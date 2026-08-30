import QtQuick
import MJQbe

// Landing page. Sprint 4 fills this with recent / favourite apps.
Rectangle {
    color: ThemeManager.bg

    Column {
        anchors.centerIn: parent
        spacing: 12

        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: qsTr("Accueil")
            color: ThemeManager.text
            font.pixelSize: 30
            font.bold: true
        }

        Text {
            anchors.horizontalCenter: parent.horizontalCenter
            text: Bridge.connected
                  ? qsTr("Connecté au core MJQbe.")
                  : qsTr("Core hors-ligne — mode dégradé.")
            color: ThemeManager.textDim
            font.pixelSize: 14
        }
    }
}
