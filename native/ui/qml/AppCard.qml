import QtQuick
import MJQbe

// App tile: rounded icon (size from settings) + truncated centered name +
// favourite star. Keyboard/remote focusable (TV navigation).
Item {
    id: root

    property int appId: 0
    property string appName: ""
    property string iconName: ""
    property string url: ""
    property bool favorite: false
    property int iconSize: 80          // 64 / 80 / 96 for small / medium / large

    signal activated()
    signal favoriteToggled()

    width: iconSize + 26
    height: iconSize + 40

    Column {
        anchors.fill: parent
        spacing: 8

        Rectangle {
            id: icon
            width: root.iconSize
            height: root.iconSize
            anchors.horizontalCenter: parent.horizontalCenter
            radius: root.iconSize * 0.22
            color: ThemeManager.surface
            border.color: root.activeFocus ? ThemeManager.accent : ThemeManager.border
            border.width: root.activeFocus ? 2 : 1

            Text {
                anchors.centerIn: parent
                text: root.appName.length > 0 ? root.appName.charAt(0).toUpperCase() : "?"
                color: ThemeManager.text
                font.pixelSize: root.iconSize * 0.38
                font.bold: true
            }

            // Favourite star (top-right).
            Rectangle {
                width: 22; height: 22; radius: 11
                anchors { top: parent.top; right: parent.right; margins: -6 }
                color: root.favorite ? ThemeManager.accent : ThemeManager.bg
                border.color: ThemeManager.border
                Text {
                    anchors.centerIn: parent
                    text: "★"
                    font.pixelSize: 12
                    color: root.favorite ? "#ffffff" : ThemeManager.textDim
                }
                TapHandler { onTapped: root.favoriteToggled() }
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

    Keys.onReturnPressed: root.activated()
    Keys.onEnterPressed: root.activated()
    Keys.onSpacePressed: root.favoriteToggled()
}
