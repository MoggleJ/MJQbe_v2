#pragma once

#include <QObject>
#include <QProcess>
#include <QString>

// Minimal embedded terminal: wraps an interactive `bash` in a QProcess and
// exposes its merged stdout/stderr as an appended text stream to QML.
// Local to the UI process (not routed through mjqbe-core).
class TerminalController : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool running READ running NOTIFY runningChanged)

public:
    explicit TerminalController(QObject *parent = nullptr);
    ~TerminalController() override;

    bool running() const { return m_process.state() != QProcess::NotRunning; }

    Q_INVOKABLE void start();
    Q_INVOKABLE void stop();
    Q_INVOKABLE void send(const QString &line); // one command; a newline is appended

signals:
    void output(const QString &chunk);
    void runningChanged();

private:
    QProcess m_process;
};
