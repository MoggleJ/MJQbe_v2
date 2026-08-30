import QtQuick
import MJQbe

// Fixed left sidebar (CDC §4.2): dynamic title, main menu, mode switch,
// Settings + live clock pinned to the bottom.
Rectangle {
    id: root
    color: ThemeManager.sidebar

    property string modeTitle: "MJ TV"
    property string currentPage: "Home"
    property bool voiceEnabled: false
    property bool voiceWakeRecent: false

    signal navigate(string page)
    signal switchMode()
    signal openSettings()

    // Right-edge separator.
    Rectangle {
        anchors.right: parent.right
        width: 1
        height: parent.height
        color: ThemeManager.border
    }

    Column {
        id: menu
        anchors { top: parent.top; left: parent.left; right: parent.right; margins: 20 }
        spacing: 6

        Row {
            spacing: 8
            bottomPadding: 20

            Text {
                text: root.modeTitle
                color: ThemeManager.text
                font.pixelSize: 24
                font.bold: true
                anchors.verticalCenter: parent.verticalCenter
            }

            // Voice wake indicator: dim dot when listening, pulses on wake word.
            Rectangle {
                anchors.verticalCenter: parent.verticalCenter
                width: 12
                height: 12
                radius: 6
                visible: root.voiceEnabled
                color: root.voiceWakeRecent ? ThemeManager.accent : ThemeManager.textDim
                opacity: root.voiceWakeRecent ? 1.0 : 0.5

                SequentialAnimation on scale {
                    running: root.voiceWakeRecent
                    loops: Animation.Infinite
                    NumberAnimation { from: 1.0; to: 1.6; duration: 400; easing.type: Easing.OutQuad }
                    NumberAnimation { from: 1.6; to: 1.0; duration: 400; easing.type: Easing.InQuad }
                }
            }
        }

        Repeater {
            model: [
                { "label": qsTr("Home"),     "page": "Home" },
                { "label": qsTr("All Apps"), "page": "AllApps" },
                { "label": qsTr("Search"),   "page": "Search" }
            ]
            delegate: SidebarButton {
                required property var modelData
                label: modelData.label
                active: root.currentPage === modelData.page
                onClicked: root.navigate(modelData.page)
            }
        }

        SidebarButton {
            label: "⇄  " + (root.modeTitle === "MJ TV" ? qsTr("MJ Desktop") : qsTr("MJ TV"))
            visible: root.modeTitle !== "MJ Dev"
            onClicked: root.switchMode()
        }

        SidebarButton {
            label: "🔒  " + qsTr("MJ Dev")
            active: root.currentPage === "Dev"
            onClicked: root.navigate("Dev")
        }
    }

    Column {
        anchors { left: parent.left; right: parent.right; bottom: parent.bottom; margins: 20 }
        spacing: 10

        SidebarButton {
            label: qsTr("Settings")
            active: root.currentPage === "Settings"
            onClicked: root.openSettings()
        }

        Clock { color: ThemeManager.textDim }

        Text {
            text: Bridge.connected ? qsTr("core: online") : qsTr("core: offline")
            color: Bridge.connected ? ThemeManager.accent : ThemeManager.textDim
            font.pixelSize: 11
        }
    }
}
