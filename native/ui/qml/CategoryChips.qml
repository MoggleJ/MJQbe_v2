import QtQuick
import MJQbe

// Horizontal row of category filter chips. `selectedId <= 0` means "All".
Flow {
    id: root

    property var categories: []      // [{ id, name, mode }]
    property int selectedId: 0
    signal selected(int categoryId)

    spacing: 8

    Repeater {
        model: [{ "id": 0, "name": qsTr("Tout") }].concat(root.categories)
        delegate: Rectangle {
            required property var modelData
            height: 30
            width: label.implicitWidth + 24
            radius: 15
            color: root.selectedId === modelData.id ? ThemeManager.accent : ThemeManager.surface
            border.color: ThemeManager.border

            Text {
                id: label
                anchors.centerIn: parent
                text: modelData.name
                font.pixelSize: 12
                color: root.selectedId === modelData.id ? "#ffffff" : ThemeManager.text
            }

            TapHandler { onTapped: root.selected(modelData.id) }
        }
    }
}
