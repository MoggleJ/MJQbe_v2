import QtQuick
import MJQbe

// Landing page: Favourites + Recent apps for the current mode.
Rectangle {
    id: page
    color: ThemeManager.bg

    property var favApps: []
    property var recentApps: []

    function reload() {
        Bridge.listApps(window.mode);   // used to resolve favourite app objects
        Bridge.listRecent(window.mode);
        Bridge.listFavorites();
    }

    Connections {
        target: Bridge
        function onAppsReceived(mode, apps) {
            if (mode !== window.mode) return;
            const favSet = window.favoriteIds;
            page.favApps = apps.filter(a => favSet.indexOf(a.id) !== -1);
        }
        function onRecentReceived(mode, apps) {
            if (mode === window.mode) page.recentApps = apps;
        }
        function onFavoritesReceived(_ids) { page.reload(); }
    }

    Component.onCompleted: reload()
    Connections { target: window; function onModeChanged() { page.reload() } }

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20

        Text {
            text: qsTr("Accueil")
            color: ThemeManager.text
            font.pixelSize: 28
            font.bold: true
        }

        Text {
            text: qsTr("Favoris")
            color: ThemeManager.textDim
            font.pixelSize: 14
            visible: page.favApps.length > 0
        }
        AppGrid {
            width: parent.width
            height: Math.min(2 * cellHeight + 4, 260)
            visible: page.favApps.length > 0
            mode: window.mode
            iconSize: window.iconSize
            favoriteIds: window.favoriteIds
            model: page.favApps
            onAppActivated: (id, url, isWeb) => window.openApp(id, url, isWeb)
            onFavoriteToggled: (id) => window.toggleFav(id)
        }

        Text {
            text: qsTr("Récents")
            color: ThemeManager.textDim
            font.pixelSize: 14
        }
        AppGrid {
            width: parent.width
            height: 260
            mode: window.mode
            iconSize: window.iconSize
            favoriteIds: window.favoriteIds
            model: page.recentApps
            emptyText: Bridge.connected ? qsTr("Aucun lancement récent.") : qsTr("Core hors-ligne.")
            onAppActivated: (id, url, isWeb) => window.openApp(id, url, isWeb)
            onFavoriteToggled: (id) => window.toggleFav(id)
        }
    }
}
