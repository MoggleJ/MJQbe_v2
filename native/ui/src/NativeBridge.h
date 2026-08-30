#pragma once

#include <QByteArray>
#include <QHash>
#include <QJsonObject>
#include <QLocalSocket>
#include <QObject>
#include <QString>
#include <QVariantList>

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

public:
    explicit NativeBridge(QObject *parent = nullptr);

    bool connected() const { return m_connected; }
    QString status() const { return m_status; }
    void setSocketPath(const QString &path) { m_socketPath = path; }

    Q_INVOKABLE void connectToCore();
    Q_INVOKABLE void ping();
    Q_INVOKABLE void listApps(const QString &mode);
    Q_INVOKABLE void listCategories(const QString &mode);
    Q_INVOKABLE void login(const QString &username, const QString &password);

signals:
    void connectedChanged();
    void statusChanged();
    void appsReceived(const QString &mode, const QVariantList &apps);
    void categoriesReceived(const QString &mode, const QVariantList &categories);
    void loginResult(bool ok, const QString &role, const QString &error);
    void coreError(const QString &code, const QString &message);

private slots:
    void onConnected();
    void onDisconnected();
    void onReadyRead();
    void onSocketError(QLocalSocket::LocalSocketError error);

private:
    struct Pending
    {
        QString method;
        QString mode; // echoed back on list results
    };

    void setStatus(const QString &status);
    void scheduleReconnect();
    QString send(const QString &method, const QJsonObject &params, const QString &mode = QString());
    void dispatch(const QJsonObject &message);

    QLocalSocket m_socket;
    QString m_socketPath;
    QString m_status{QStringLiteral("disconnected")};
    bool m_connected{false};
    QByteArray m_buffer;
    quint64 m_nextId{1};
    QHash<QString, Pending> m_pending;
    bool m_reconnectScheduled{false};
};
