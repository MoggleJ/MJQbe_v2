import QtQuick
import MJQbe

// Sprint 3: theme selector (10 options). Sprint 4 adds layout + icon_size and
// persistence through the core.
Rectangle {
    color: ThemeManager.bg

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 16

        Text {
            text: qsTr("Paramètres")
            color: ThemeManager.text
            font.pixelSize: 24
            font.bold: true
        }

        Text {
            text: qsTr("Thème")
            color: ThemeManager.textDim
            font.pixelSize: 14
        }

        Flow {
            width: parent.width
            spacing: 10

            Repeater {
                model: ThemeManager.names
                delegate: Rectangle {
                    required property string modelData
                    width: 132
                    height: 44
                    radius: 8
                    color: ThemeManager.current === modelData ? ThemeManager.accent : ThemeManager.surface
                    border.color: ThemeManager.border

                    Text {
                        anchors.centerIn: parent
                        text: modelData
                        color: ThemeManager.current === modelData ? "#ffffff" : ThemeManager.text
                        font.pixelSize: 12
                    }

                    TapHandler { onTapped: ThemeManager.setTheme(modelData) }
                }
            }
        }
    }
}
