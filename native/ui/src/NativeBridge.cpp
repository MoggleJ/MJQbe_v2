#include "NativeBridge.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonValue>
#include <QTimer>

namespace {
constexpr int kReconnectDelayMs = 2000;

QString defaultSocketPath()
{
    const QByteArray fromEnv = qgetenv("MJQBE_NATIVE_SOCKET");
    if (!fromEnv.isEmpty())
        return QString::fromLocal8Bit(fromEnv);
    return QStringLiteral("/run/mjqbe/native.sock");
}
} // namespace

NativeBridge::NativeBridge(QObject *parent)
    : QObject(parent), m_socketPath(defaultSocketPath())
{
    connect(&m_socket, &QLocalSocket::connected, this, &NativeBridge::onConnected);
    connect(&m_socket, &QLocalSocket::disconnected, this, &NativeBridge::onDisconnected);
    connect(&m_socket, &QLocalSocket::readyRead, this, &NativeBridge::onReadyRead);
    connect(&m_socket, &QLocalSocket::errorOccurred, this, &NativeBridge::onSocketError);
}

void NativeBridge::connectToCore()
{
    if (m_socket.state() != QLocalSocket::UnconnectedState)
        return;
    setStatus(QStringLiteral("connecting"));
    m_socket.connectToServer(m_socketPath);
}

void NativeBridge::onConnected()
{
    m_buffer.clear();
    m_connected = true;
    emit connectedChanged();
    setStatus(QStringLiteral("connected"));
    fetchSession(); // resolve the current user, then the UI can load prefs/favourites
}

void NativeBridge::onDisconnected()
{
    m_connected = false;
    emit connectedChanged();
    setStatus(QStringLiteral("disconnected"));
    m_pending.clear();
    scheduleReconnect();
}

void NativeBridge::onSocketError(QLocalSocket::LocalSocketError error)
{
    Q_UNUSED(error);
    setStatus(QStringLiteral("error: ") + m_socket.errorString());
    if (m_connected) {
        m_connected = false;
        emit connectedChanged();
    }
    scheduleReconnect();
}

void NativeBridge::scheduleReconnect()
{
    if (m_reconnectScheduled)
        return;
    m_reconnectScheduled = true;
    QTimer::singleShot(kReconnectDelayMs, this, [this] {
        m_reconnectScheduled = false;
        if (!m_connected)
            connectToCore();
    });
}

void NativeBridge::setStatus(const QString &status)
{
    if (m_status == status)
        return;
    m_status = status;
    emit statusChanged();
}

QString NativeBridge::send(const QString &method, const QJsonObject &params, const QString &arg)
{
    const QString id = QString::number(m_nextId++);
    if (m_socket.state() != QLocalSocket::ConnectedState) {
        emit coreError(QStringLiteral("not_connected"),
                       QStringLiteral("core socket is not connected"));
        return id;
    }

    m_pending.insert(id, Pending{method, arg});

    QJsonObject req;
    req.insert(QStringLiteral("id"), id);
    req.insert(QStringLiteral("method"), method);
    req.insert(QStringLiteral("params"), params);

    QByteArray line = QJsonDocument(req).toJson(QJsonDocument::Compact);
    line.append('\n');
    m_socket.write(line);
    m_socket.flush();
    return id;
}

void NativeBridge::onReadyRead()
{
    m_buffer.append(m_socket.readAll());

    int newline;
    while ((newline = m_buffer.indexOf('\n')) != -1) {
        const QByteArray raw = m_buffer.left(newline);
        m_buffer.remove(0, newline + 1);
        if (raw.trimmed().isEmpty())
            continue;

        QJsonParseError err;
        const QJsonDocument doc = QJsonDocument::fromJson(raw, &err);
        if (err.error != QJsonParseError::NoError || !doc.isObject()) {
            emit coreError(QStringLiteral("bad_response"), err.errorString());
            continue;
        }
        dispatch(doc.object());
    }
}

void NativeBridge::dispatch(const QJsonObject &message)
{
    const QString id = message.value(QStringLiteral("id")).toString();
    const bool ok = message.value(QStringLiteral("ok")).toBool();
    const Pending pending = m_pending.take(id);
    const QString &method = pending.method;

    if (!ok) {
        const QJsonObject error = message.value(QStringLiteral("error")).toObject();
        const QString code = error.value(QStringLiteral("code")).toString();
        const QString msg = error.value(QStringLiteral("message")).toString();
        if (method == QStringLiteral("auth.login"))
            emit loginResult(false, QString(), msg);
        else
            emit coreError(code, msg);
        return;
    }

    const QJsonValue data = message.value(QStringLiteral("data"));

    if (method == QStringLiteral("apps.list")) {
        emit appsReceived(pending.arg, data.toArray().toVariantList());
    } else if (method == QStringLiteral("apps.recent")) {
        emit recentReceived(pending.arg, data.toArray().toVariantList());
    } else if (method == QStringLiteral("categories.list")) {
        emit categoriesReceived(pending.arg, data.toArray().toVariantList());
    } else if (method == QStringLiteral("favorites.list")) {
        emit favoritesReceived(data.toArray().toVariantList());
    } else if (method == QStringLiteral("favorites.toggle")) {
        const QJsonObject o = data.toObject();
        emit favoriteToggled(o.value(QStringLiteral("app_id")).toInt(),
                             o.value(QStringLiteral("favorited")).toBool());
    } else if (method == QStringLiteral("settings.get")
               || method == QStringLiteral("settings.update")) {
        emit settingsReceived(data.toObject().toVariantMap());
    } else if (method == QStringLiteral("session.current")) {
        const QJsonObject o = data.toObject();
        m_userId = o.value(QStringLiteral("user_id")).toInt(m_userId);
        m_userName = o.value(QStringLiteral("username")).toString(m_userName);
        emit sessionChanged();
    } else if (method == QStringLiteral("auth.login")) {
        const QJsonObject o = data.toObject();
        m_userId = o.value(QStringLiteral("user_id")).toInt(m_userId);
        m_userName = o.value(QStringLiteral("username")).toString(m_userName);
        emit sessionChanged();
        emit loginResult(true, o.value(QStringLiteral("role")).toString(), QString());
    }
    // ping / health: nothing to surface.
}

void NativeBridge::ping()
{
    send(QStringLiteral("ping"), {});
}

void NativeBridge::fetchSession()
{
    send(QStringLiteral("session.current"), {});
}

void NativeBridge::listApps(const QString &mode, int categoryId)
{
    QJsonObject p{{QStringLiteral("mode"), mode}};
    if (categoryId > 0)
        p.insert(QStringLiteral("category_id"), categoryId);
    send(QStringLiteral("apps.list"), p, mode);
}

void NativeBridge::listRecent(const QString &mode)
{
    send(QStringLiteral("apps.recent"),
         QJsonObject{{QStringLiteral("user_id"), m_userId}, {QStringLiteral("mode"), mode}}, mode);
}

void NativeBridge::listCategories(const QString &mode)
{
    send(QStringLiteral("categories.list"), QJsonObject{{QStringLiteral("mode"), mode}}, mode);
}

void NativeBridge::listFavorites()
{
    send(QStringLiteral("favorites.list"), QJsonObject{{QStringLiteral("user_id"), m_userId}});
}

void NativeBridge::toggleFavorite(int appId)
{
    send(QStringLiteral("favorites.toggle"),
         QJsonObject{{QStringLiteral("user_id"), m_userId}, {QStringLiteral("app_id"), appId}});
}

void NativeBridge::getSettings()
{
    send(QStringLiteral("settings.get"), QJsonObject{{QStringLiteral("user_id"), m_userId}});
}

void NativeBridge::updateSettings(const QVariantMap &patch)
{
    QJsonObject p = QJsonObject::fromVariantMap(patch);
    p.insert(QStringLiteral("user_id"), m_userId);
    send(QStringLiteral("settings.update"), p);
}

void NativeBridge::login(const QString &username, const QString &password)
{
    send(QStringLiteral("auth.login"),
         QJsonObject{{QStringLiteral("username"), username},
                     {QStringLiteral("password"), password}});
}
