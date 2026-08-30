import QtQuick
import QtQuick.Controls
import MJQbe

// Live search across the current mode's apps.
Rectangle {
    id: page
    color: ThemeManager.bg

    property var allApps: []
    property string query: ""
    readonly property var results: {
        const q = query.toLowerCase();
        return q.length === 0 ? [] : allApps.filter(a => a.name.toLowerCase().indexOf(q) !== -1);
    }

    Connections {
        target: Bridge
        function onAppsReceived(mode, apps) { if (mode === window.mode) page.allApps = apps; }
    }
    Connections { target: window; function onModeChanged() { Bridge.listApps(window.mode) } }
    Component.onCompleted: Bridge.listApps(window.mode)

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 16

        TextField {
            id: field
            width: parent.width
            placeholderText: qsTr("Rechercher une application…")
            color: ThemeManager.text
            focus: true
            onTextChanged: page.query = text
        }

        AppGrid {
            width: parent.width
            height: parent.height - field.height - 16
            mode: window.mode
            iconSize: window.iconSize
            favoriteIds: window.favoriteIds
            model: page.results
            emptyText: page.query.length === 0 ? qsTr("Tapez pour rechercher.") : qsTr("Aucun résultat.")
            onAppActivated: (id, url, isWeb) => window.openApp(id, url, isWeb)
            onFavoriteToggled: (id) => window.toggleFav(id)
        }
    }
}
