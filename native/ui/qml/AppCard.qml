import QtQuick
import MJQbe

// App tile: rounded 80x80 icon placeholder + truncated centered name.
// Sprint 4 replaces the placeholder with real icons and adds favourites.
Item {
    id: root

    property string appName: ""
    property string iconName: ""
    property string url: ""
    signal activated()

    width: 100
    height: 120

    Column {
        anchors.fill: parent
        spacing: 8

        Rectangle {
            id: icon
            width: 80
            height: 80
            anchors.horizontalCenter: parent.horizontalCenter
            radius: 18
            color: ThemeManager.surface
            border.color: focusRing.visible ? ThemeManager.accent : ThemeManager.border
            border.width: focusRing.visible ? 2 : 1

            Text {
                anchors.centerIn: parent
                text: root.appName.length > 0 ? root.appName.charAt(0).toUpperCase() : "?"
                color: ThemeManager.text
                font.pixelSize: 30
                font.bold: true
            }

            TapHandler { onTapped: root.activated() }
        }

        Text {
            width: parent.width
            horizontalAlignment: Text.AlignHCenter
            text: root.appName
            color: ThemeManager.textDim
            font.pixelSize: 12
            elide: Text.ElideRight
            maximumLineCount: 1
        }
    }

    // Keyboard / remote navigation (expanded in Sprint 4).
    Rectangle { id: focusRing; visible: root.activeFocus; color: "transparent" }
    Keys.onReturnPressed: root.activated()
    Keys.onEnterPressed: root.activated()
}
