import QtQuick
import MJQbe

// Theme (10) + layout (grid/list) + icon size (small/medium/large).
// Every change is persisted through the core (settings.update).
Rectangle {
    id: page
    color: ThemeManager.bg

    function choose(field, value) {
        var patch = {};
        patch[field] = value;
        Bridge.updateSettings(patch);
    }

    component OptionRow: Column {
        property string label: ""
        property var options: []
        property string current: ""
        property string field: ""
        width: parent.width
        spacing: 8

        Text { text: label; color: ThemeManager.textDim; font.pixelSize: 14 }

        Flow {
            width: parent.width
            spacing: 10
            Repeater {
                model: options
                delegate: Rectangle {
                    required property string modelData
                    height: 40
                    width: chipText.implicitWidth + 28
                    radius: 8
                    color: modelData === current ? ThemeManager.accent : ThemeManager.surface
                    border.color: ThemeManager.border
                    Text {
                        id: chipText
                        anchors.centerIn: parent
                        text: modelData
                        font.pixelSize: 12
                        color: modelData === current ? "#ffffff" : ThemeManager.text
                    }
                    TapHandler { onTapped: page.choose(field, modelData) }
                }
            }
        }
    }

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 22

        Text {
            text: qsTr("Paramètres")
            color: ThemeManager.text
            font.pixelSize: 24
            font.bold: true
        }

        OptionRow {
            label: qsTr("Thème")
            options: ThemeManager.names
            current: ThemeManager.current
            field: "theme"
        }
        OptionRow {
            label: qsTr("Disposition")
            options: ["grid", "list"]
            current: window.layout
            field: "layout"
        }
        OptionRow {
            label: qsTr("Taille des icônes")
            options: ["small", "medium", "large"]
            current: window.iconSize
            field: "icon_size"
        }
    }
}
