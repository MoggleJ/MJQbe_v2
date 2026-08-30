#pragma once

#include <QByteArray>
#include <QHash>
#include <QJsonObject>
#include <QLocalSocket>
#include <QObject>
#include <QString>
#include <QVariantList>
#include <QVariantMap>

// Client side of the native IPC channel: a QLocalSocket connected to the
// mjqbe-core Unix socket, speaking newline-delimited JSON.
//
// Exposed to QML as the context property `Bridge`. Requests are fire-and-forget
// from QML's point of view; results arrive as signals. If the core is
// unreachable the bridge keeps retrying and simply stays `connected == false`
// so the UI can run in degraded mode.
class NativeBridge : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool connected READ connected NOTIFY connectedChanged)
    Q_PROPERTY(QString status READ status NOTIFY statusChanged)
    Q_PROPERTY(int userId READ userId NOTIFY sessionChanged)
    Q_PROPERTY(QString userName READ userName NOTIFY sessionChanged)

public:
    explicit NativeBridge(QObject *parent = nullptr);

    bool connected() const { return m_connected; }
    QString status() const { return m_status; }
    int userId() const { return m_userId; }
    QString userName() const { return m_userName; }
    void setSocketPath(const QString &path) { m_socketPath = path; }

    Q_INVOKABLE void connectToCore();
    Q_INVOKABLE void ping();
    Q_INVOKABLE void fetchSession();
    Q_INVOKABLE void listApps(const QString &mode, int categoryId = -1);
    Q_INVOKABLE void listRecent(const QString &mode);
    Q_INVOKABLE void listCategories(const QString &mode);
    Q_INVOKABLE void listFavorites();
    Q_INVOKABLE void toggleFavorite(int appId);
    Q_INVOKABLE void getSettings();
    Q_INVOKABLE void updateSettings(const QVariantMap &patch);
    Q_INVOKABLE void login(const QString &username, const QString &password);

    // --- Dev mode --------------------------------------------------------
    Q_INVOKABLE void verify(const QString &password);         // → verifyResult(token)
    Q_INVOKABLE void systemSnapshot();
    Q_INVOKABLE void listProcesses(int limit = 60);
    Q_INVOKABLE void killProcess(const QString &token, int pid);
    Q_INVOKABLE void niceProcess(const QString &token, int pid, int niceness);
    Q_INVOKABLE void listContainers();
    Q_INVOKABLE void dockerStart(const QString &token, const QString &id);
    Q_INVOKABLE void dockerStop(const QString &token, const QString &id);
    Q_INVOKABLE void avStatus();
    Q_INVOKABLE void avSend(const QString &token, const QString &action);
    Q_INVOKABLE void voiceStatus();
    Q_INVOKABLE void voiceSimulate(const QString &text);
    Q_INVOKABLE void voiceSetEnabled(const QString &token, bool enabled);

signals:
    void connectedChanged();
    void statusChanged();
    void sessionChanged();
    void appsReceived(const QString &mode, const QVariantList &apps);
    void recentReceived(const QString &mode, const QVariantList &apps);
    void categoriesReceived(const QString &mode, const QVariantList &categories);
    void favoritesReceived(const QVariantList &appIds);
    void favoriteToggled(int appId, bool favorited);
    void settingsReceived(const QVariantMap &settings);
    void loginResult(bool ok, const QString &role, const QString &error);
    void coreError(const QString &code, const QString &message);

    void verifyResult(bool ok, const QString &token, const QString &error);
    void snapshotReceived(const QVariantMap &snapshot);
    void processesReceived(const QVariantList &processes);
    void containersReceived(const QVariantList &containers);
    void devActionResult(const QString &action, bool ok, const QString &error);
    void avStatusReceived(const QVariantMap &status);
    void voiceStatusReceived(const QVariantMap &status);
    void voiceResult(const QVariantMap &result);

private slots:
    void onConnected();
    void onDisconnected();
    void onReadyRead();
    void onSocketError(QLocalSocket::LocalSocketError error);

private:
    struct Pending
    {
        QString method;
        QString arg; // echoed context (mode, ...)
    };

    void setStatus(const QString &status);
    void scheduleReconnect();
    QString send(const QString &method, const QJsonObject &params, const QString &arg = QString());
    void dispatch(const QJsonObject &message);

    QLocalSocket m_socket;
    QString m_socketPath;
    QString m_status{QStringLiteral("disconnected")};
    bool m_connected{false};
    int m_userId{1}; // default to admin so favourites work without explicit login in dev
    QString m_userName{QStringLiteral("admin")};
    QByteArray m_buffer;
    quint64 m_nextId{1};
    QHash<QString, Pending> m_pending;
    bool m_reconnectScheduled{false};
};
