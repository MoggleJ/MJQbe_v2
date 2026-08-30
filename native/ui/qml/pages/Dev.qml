import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import MJQbe

// Dev mode: admin re-auth gate → monitoring + process/Docker control + terminal.
// Every destructive action re-prompts for the admin password (CDC §8).
Rectangle {
    id: page
    color: ThemeManager.bg

    property bool unlocked: false
    property var snap: ({})
    property var procs: []
    property var containers: []
    property var av: ({})

    // Pending destructive action, resolved by the re-auth dialog.
    property var pendingAction: null   // function(token)

    Timer {
        id: poll
        interval: 2000
        repeat: true
        running: page.unlocked && page.visible
        triggeredOnStart: true
        onTriggered: {
            Bridge.systemSnapshot();
            Bridge.listProcesses(60);
            Bridge.listContainers();
            Bridge.avStatus();
        }
    }

    Connections {
        target: Bridge
        function onSnapshotReceived(s) { page.snap = s; }
        function onProcessesReceived(p) { page.procs = p; }
        function onContainersReceived(c) { page.containers = c; }
        function onAvStatusReceived(s) { page.av = s; }
        function onVerifyResult(ok, token, error) {
            if (!ok) { gateMsg.text = qsTr("Échec : ") + error; return; }
            if (!page.unlocked) { page.unlocked = true; gateMsg.text = ""; }
            else if (page.pendingAction) { var a = page.pendingAction; page.pendingAction = null; a(token); }
        }
        function onDevActionResult(action, ok, error) {
            actionMsg.text = ok ? (action + " ✓") : (action + " — " + error);
        }
    }

    function requireReauth(fn) {
        page.pendingAction = fn;
        reauthPass.text = "";
        reauthDialog.open();
    }

    Component.onCompleted: Terminal.start()

    // ---------- gate ----------
    Column {
        anchors.centerIn: parent
        spacing: 12
        width: 300
        visible: !page.unlocked

        Text {
            text: qsTr("Mode Dev — authentification admin requise")
            color: ThemeManager.text
            font.pixelSize: 18
            font.bold: true
            wrapMode: Text.Wrap
            width: parent.width
        }
        TextField {
            id: gatePass
            width: parent.width
            placeholderText: qsTr("Mot de passe admin")
            echoMode: TextInput.Password
            color: ThemeManager.text
            onAccepted: Bridge.verify(text)
        }
        Button { text: qsTr("Déverrouiller"); onClicked: Bridge.verify(gatePass.text) }
        Text { id: gateMsg; color: "#e5534b"; font.pixelSize: 12 }
    }

    // ---------- dashboard ----------
    ScrollView {
        anchors.fill: parent
        anchors.margins: 20
        visible: page.unlocked
        clip: true

        ColumnLayout {
            width: page.width - 56
            spacing: 20

            Text {
                text: qsTr("MJ Dev")
                color: ThemeManager.text
                font.pixelSize: 24
                font.bold: true
            }

            // Monitoring
            Flow {
                Layout.fillWidth: true
                spacing: 24
                Gauge {
                    label: "CPU"
                    percent: page.snap.cpu_percent || 0
                    detail: "load " + ((page.snap.load_avg || [0,0,0]).map(x => x.toFixed(2)).join(" "))
                }
                Gauge {
                    label: "RAM"
                    percent: page.snap.mem_total_kb ? 100 * page.snap.mem_used_kb / page.snap.mem_total_kb : 0
                    detail: Math.round((page.snap.mem_used_kb||0)/1024) + " / " + Math.round((page.snap.mem_total_kb||0)/1024) + " Mo"
                }
                Gauge {
                    label: qsTr("Disque /")
                    percent: page.snap.disk_total_kb ? 100 * page.snap.disk_used_kb / page.snap.disk_total_kb : 0
                    detail: Math.round((page.snap.disk_used_kb||0)/1048576) + " / " + Math.round((page.snap.disk_total_kb||0)/1048576) + " Go"
                }
                Gauge {
                    label: qsTr("Température")
                    percent: page.snap.temp_celsius ? Math.min(100, page.snap.temp_celsius / 90 * 100) : 0
                    detail: page.snap.temp_celsius ? page.snap.temp_celsius.toFixed(1) + " °C" : qsTr("n/a")
                }
                Gauge {
                    label: qsTr("Réseau")
                    percent: 0
                    detail: "↓ " + fmtRate(page.snap.net_rx_bytes_per_s) + "   ↑ " + fmtRate(page.snap.net_tx_bytes_per_s)
                }
            }

            Text { id: actionMsg; color: ThemeManager.textDim; font.pixelSize: 12 }

            // Processes
            Text { text: qsTr("Processus (top RSS)"); color: ThemeManager.textDim; font.pixelSize: 14 }
            ListView {
                Layout.fillWidth: true
                Layout.preferredHeight: 220
                clip: true
                model: page.procs
                delegate: Row {
                    required property var modelData
                    width: ListView.view.width
                    height: 26
                    spacing: 12
                    Text { text: modelData.pid; color: ThemeManager.textDim; font.pixelSize: 12; width: 60 }
                    Text { text: modelData.name; color: ThemeManager.text; font.pixelSize: 12; width: 200; elide: Text.ElideRight }
                    Text { text: modelData.state; color: ThemeManager.textDim; font.pixelSize: 12; width: 30 }
                    Text { text: Math.round(modelData.mem_rss_kb/1024) + " Mo"; color: ThemeManager.textDim; font.pixelSize: 12; width: 80 }
                    Text { text: "nice " + modelData.nice; color: ThemeManager.textDim; font.pixelSize: 12; width: 60 }
                    Button {
                        text: qsTr("kill")
                        onClicked: { var pid = modelData.pid; page.requireReauth(t => Bridge.killProcess(t, pid)); }
                    }
                    Button {
                        text: "nice +5"
                        onClicked: { var pid = modelData.pid; var n = modelData.nice + 5; page.requireReauth(t => Bridge.niceProcess(t, pid, n)); }
                    }
                }
            }

            // Docker
            Text { text: qsTr("Conteneurs Docker"); color: ThemeManager.textDim; font.pixelSize: 14 }
            ListView {
                Layout.fillWidth: true
                Layout.preferredHeight: 160
                clip: true
                model: page.containers
                delegate: Row {
                    required property var modelData
                    width: ListView.view.width
                    height: 26
                    spacing: 12
                    Text { text: modelData.name; color: ThemeManager.text; font.pixelSize: 12; width: 200; elide: Text.ElideRight }
                    Text { text: modelData.image; color: ThemeManager.textDim; font.pixelSize: 12; width: 220; elide: Text.ElideRight }
                    Text { text: modelData.state; color: modelData.state === "running" ? ThemeManager.accent : ThemeManager.textDim; font.pixelSize: 12; width: 80 }
                    Button {
                        text: qsTr("start")
                        onClicked: { var id = modelData.id; page.requireReauth(t => Bridge.dockerStart(t, id)); }
                    }
                    Button {
                        text: qsTr("stop")
                        onClicked: { var id = modelData.id; page.requireReauth(t => Bridge.dockerStop(t, id)); }
                    }
                }
            }

            // AV control (HDMI-CEC / IR / Bluetooth)
            Text {
                text: qsTr("AV") + "  —  cec:" + (page.av.cec ? "✓" : "✗")
                      + "  ir:" + (page.av.ir ? "✓" : "✗")
                      + "  bt:" + (page.av.bt ? "✓" : "✗")
                color: ThemeManager.textDim
                font.pixelSize: 14
            }
            Flow {
                Layout.fillWidth: true
                spacing: 10
                Repeater {
                    model: [
                        { "label": qsTr("Allume TV"), "action": "tv_on" },
                        { "label": qsTr("Éteins TV"), "action": "tv_off" },
                        { "label": "PS4 on", "action": "ps4_on" },
                        { "label": "PS4 off", "action": "ps4_off" }
                    ]
                    delegate: Button {
                        required property var modelData
                        text: modelData.label
                        onClicked: {
                            var act = modelData.action;
                            page.requireReauth(t => Bridge.avSend(t, act));
                        }
                    }
                }
            }

            // Terminal
            Row {
                spacing: 12
                Text { text: qsTr("Terminal"); color: ThemeManager.textDim; font.pixelSize: 14; anchors.verticalCenter: parent.verticalCenter }
                Button {
                    text: qsTr("Interface graphique Pi")
                    onClicked: Qt.openUrlExternally("vnc://localhost:5900")
                }
            }
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 200
                color: "#000000"
                border.color: ThemeManager.border
                ScrollView {
                    id: termScroll
                    anchors.fill: parent
                    anchors.margins: 6
                    TextArea {
                        id: termView
                        readOnly: true
                        wrapMode: TextArea.WrapAnywhere
                        color: "#d0d0d0"
                        font.family: "monospace"
                        font.pixelSize: 12
                        background: null
                    }
                }
            }
            TextField {
                id: termInput
                Layout.fillWidth: true
                placeholderText: "$ …"
                color: ThemeManager.text
                onAccepted: { Terminal.send(text); termView.append("$ " + text); text = ""; }
            }
        }
    }

    Connections {
        target: Terminal
        function onOutput(chunk) {
            termView.insert(termView.length, chunk);
            termView.cursorPosition = termView.length;
        }
    }

    function fmtRate(bytesPerSec) {
        var b = bytesPerSec || 0;
        if (b > 1048576) return (b / 1048576).toFixed(1) + " Mo/s";
        if (b > 1024) return (b / 1024).toFixed(0) + " Ko/s";
        return b + " o/s";
    }

    // ---------- re-auth dialog ----------
    Dialog {
        id: reauthDialog
        anchors.centerIn: parent
        modal: true
        title: qsTr("Ré-authentification requise")
        standardButtons: Dialog.Ok | Dialog.Cancel
        onAccepted: Bridge.verify(reauthPass.text)
        onRejected: page.pendingAction = null

        ColumnLayout {
            TextField {
                id: reauthPass
                Layout.preferredWidth: 240
                placeholderText: qsTr("Mot de passe admin")
                echoMode: TextInput.Password
                onAccepted: { reauthDialog.accept(); }
            }
        }
    }
}
