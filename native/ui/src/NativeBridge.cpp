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

QString NativeBridge::send(const QString &method, const QJsonObject &params, const QString &mode)
{
    const QString id = QString::number(m_nextId++);
    if (m_socket.state() != QLocalSocket::ConnectedState) {
        emit coreError(QStringLiteral("not_connected"),
                       QStringLiteral("core socket is not connected"));
        return id;
    }

    m_pending.insert(id, Pending{method, mode});

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

    if (!ok) {
        const QJsonObject error = message.value(QStringLiteral("error")).toObject();
        const QString code = error.value(QStringLiteral("code")).toString();
        const QString msg = error.value(QStringLiteral("message")).toString();
        if (pending.method == QStringLiteral("auth.login"))
            emit loginResult(false, QString(), msg);
        else
            emit coreError(code, msg);
        return;
    }

    const QJsonValue data = message.value(QStringLiteral("data"));

    if (pending.method == QStringLiteral("apps.list")) {
        emit appsReceived(pending.mode, data.toArray().toVariantList());
    } else if (pending.method == QStringLiteral("categories.list")) {
        emit categoriesReceived(pending.mode, data.toArray().toVariantList());
    } else if (pending.method == QStringLiteral("auth.login")) {
        emit loginResult(true, data.toObject().value(QStringLiteral("role")).toString(), QString());
    }
    // ping / health: nothing to surface for now.
}

void NativeBridge::ping()
{
    send(QStringLiteral("ping"), {});
}

void NativeBridge::listApps(const QString &mode)
{
    send(QStringLiteral("apps.list"), QJsonObject{{QStringLiteral("mode"), mode}}, mode);
}

void NativeBridge::listCategories(const QString &mode)
{
    send(QStringLiteral("categories.list"), QJsonObject{{QStringLiteral("mode"), mode}}, mode);
}

void NativeBridge::login(const QString &username, const QString &password)
{
    send(QStringLiteral("auth.login"),
         QJsonObject{{QStringLiteral("username"), username},
                     {QStringLiteral("password"), password}});
}
