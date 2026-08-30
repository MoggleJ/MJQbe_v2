import QtQuick
import MJQbe

// One clickable row in the sidebar menu.
Rectangle {
    id: root

    property string label: ""
    property bool active: false
    signal clicked()

    width: parent ? parent.width : 200
    height: 40
    radius: 8
    color: active ? ThemeManager.accent
                  : (hover.hovered ? ThemeManager.surface : "transparent")

    Behavior on color { ColorAnimation { duration: 120 } }

    Text {
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: parent.left
        anchors.leftMargin: 12
        anchors.right: parent.right
        anchors.rightMargin: 8
        text: root.label
        elide: Text.ElideRight
        color: root.active ? "#ffffff" : ThemeManager.text
        font.pixelSize: 15
    }

    HoverHandler { id: hover }
    TapHandler { onTapped: root.clicked() }

    Keys.onReturnPressed: root.clicked()
    Keys.onEnterPressed: root.clicked()
    Keys.onSpacePressed: root.clicked()
}
