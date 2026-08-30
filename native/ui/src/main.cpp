// MJQbe native UI — Qt6 / QML entry point.
//
// Loads the QML root, wires a NativeBridge (Unix-socket client to mjqbe-core)
// into the QML context. Fullscreen by default; pass --windowed for desktop dev.

#include <QCommandLineOption>
#include <QCommandLineParser>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QUrl>

#include "NativeBridge.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    app.setApplicationName(QStringLiteral("MJQbe Native"));
    app.setOrganizationName(QStringLiteral("MJQbe"));
    app.setApplicationVersion(QStringLiteral("0.1.0"));

    QCommandLineParser parser;
    parser.setApplicationDescription(QStringLiteral("MJQbe embedded hub — native app"));
    parser.addHelpOption();
    parser.addVersionOption();
    const QCommandLineOption windowedOpt(
        QStringLiteral("windowed"),
        QStringLiteral("Run in a window instead of fullscreen (desktop dev)."));
    const QCommandLineOption socketOpt(
        QStringLiteral("socket"),
        QStringLiteral("Path to the mjqbe-core IPC socket."),
        QStringLiteral("path"));
    parser.addOption(windowedOpt);
    parser.addOption(socketOpt);
    parser.process(app);

    NativeBridge bridge;
    if (parser.isSet(socketOpt))
        bridge.setSocketPath(parser.value(socketOpt));
    bridge.connectToCore();

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(QStringLiteral("Bridge"), &bridge);
    engine.rootContext()->setContextProperty(QStringLiteral("startFullScreen"),
                                             !parser.isSet(windowedOpt));

    const QUrl rootUrl(QStringLiteral("qrc:/qt/qml/MJQbe/qml/Main.qml"));
    QObject::connect(
        &engine, &QQmlApplicationEngine::objectCreated, &app,
        [rootUrl](QObject *obj, const QUrl &objUrl) {
            if (!obj && rootUrl == objUrl)
                QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection);

    engine.load(rootUrl);
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
