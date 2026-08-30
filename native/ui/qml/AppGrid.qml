import QtQuick
import MJQbe

// Reusable grid of AppCards.
//  - `mode` "tv" → larger cells + wrap key navigation (remote friendly)
//  - `mode` "desktop" → denser cells
//  - `iconSize` from settings (small/medium/large)
//  - `favoriteIds` : array of app ids currently favourited
GridView {
    id: grid

    property string mode: "tv"
    property string iconSize: "medium"
    property var favoriteIds: []
    property string emptyText: qsTr("Aucune application.")

    signal appActivated(int appId, string url, bool isWeb)
    signal favoriteToggled(int appId)

    readonly property int iconPx: iconSize === "small" ? 64 : (iconSize === "large" ? 96 : 80)

    cellWidth: mode === "tv" ? iconPx + 60 : iconPx + 26
    cellHeight: iconPx + (mode === "tv" ? 56 : 40)
    clip: true
    focus: true
    keyNavigationWraps: true
    highlightMoveDuration: 120

    // `model` is always a JS array of app objects → delegates get `modelData`.
    delegate: AppCard {
        required property var modelData
        required property int index
        width: grid.cellWidth
        height: grid.cellHeight
        appId: modelData.id
        appName: modelData.name
        iconName: modelData.icon ? modelData.icon : ""
        url: modelData.url ? modelData.url : ""
        iconSize: grid.iconPx
        favorite: grid.favoriteIds.indexOf(modelData.id) !== -1
        focus: index === grid.currentIndex
        onActivated: grid.appActivated(modelData.id, url, modelData.is_web === true)
        onFavoriteToggled: grid.favoriteToggled(modelData.id)
    }

    Text {
        anchors.centerIn: parent
        visible: grid.count === 0
        text: grid.emptyText
        color: ThemeManager.textDim
    }
}
