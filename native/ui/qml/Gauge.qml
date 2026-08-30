import QtQuick
import MJQbe

// Compact labelled bar gauge (0..100 %).
Column {
    id: root

    property string label: ""
    property real percent: 0
    property string detail: ""

    spacing: 4
    width: 200

    Row {
        width: parent.width
        Text {
            text: root.label
            color: ThemeManager.textDim
            font.pixelSize: 12
            width: parent.width - valueText.width
        }
        Text {
            id: valueText
            text: Math.round(root.percent) + "%"
            color: ThemeManager.text
            font.pixelSize: 12
            font.bold: true
        }
    }

    Rectangle {
        width: parent.width
        height: 8
        radius: 4
        color: ThemeManager.surface
        border.color: ThemeManager.border

        Rectangle {
            width: Math.max(0, Math.min(1, root.percent / 100)) * parent.width
            height: parent.height
            radius: 4
            color: root.percent > 90 ? "#e5534b"
                                     : (root.percent > 70 ? "#d9a441" : ThemeManager.accent)
            Behavior on width { NumberAnimation { duration: 300 } }
        }
    }

    Text {
        text: root.detail
        color: ThemeManager.textDim
        font.pixelSize: 10
        visible: detail.length > 0
    }
}
