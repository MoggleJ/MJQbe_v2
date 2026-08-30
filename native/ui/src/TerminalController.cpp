#include "TerminalController.h"

#include <QProcessEnvironment>

TerminalController::TerminalController(QObject *parent) : QObject(parent)
{
    m_process.setProcessChannelMode(QProcess::MergedChannels);

    connect(&m_process, &QProcess::readyReadStandardOutput, this, [this] {
        emit output(QString::fromLocal8Bit(m_process.readAllStandardOutput()));
    });
    connect(&m_process, &QProcess::stateChanged, this, &TerminalController::runningChanged);
    connect(&m_process, &QProcess::errorOccurred, this, [this](QProcess::ProcessError) {
        emit output(QStringLiteral("\n[terminal] %1\n").arg(m_process.errorString()));
    });
}

TerminalController::~TerminalController()
{
    stop();
}

void TerminalController::start()
{
    if (running())
        return;
    QProcessEnvironment env = QProcessEnvironment::systemEnvironment();
    env.insert(QStringLiteral("TERM"), QStringLiteral("dumb"));
    env.insert(QStringLiteral("PS1"), QStringLiteral("mjqbe:\\w$ "));
    m_process.setProcessEnvironment(env);
    m_process.start(QStringLiteral("/bin/bash"), {QStringLiteral("-i")});
}

void TerminalController::stop()
{
    if (!running())
        return;
    m_process.terminate();
    if (!m_process.waitForFinished(1500))
        m_process.kill();
}

void TerminalController::send(const QString &line)
{
    if (!running())
        start();
    m_process.write(line.toLocal8Bit() + '\n');
}
