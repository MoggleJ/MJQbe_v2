import QtQuick
import QtQuick.Controls
import MJQbe

// All apps for the current mode: category chips + live text filter + grid.
// TV mode → wide cells + remote key navigation; Desktop → dense.
Rectangle {
    id: page
    color: ThemeManager.bg

    property var allApps: []
    property var categories: []
    property int categoryId: 0
    property string query: ""

    readonly property var filtered: {
        const q = query.toLowerCase();
        return allApps.filter(a =>
            (categoryId <= 0 || a.category_id === categoryId) &&
            (q.length === 0 || a.name.toLowerCase().indexOf(q) !== -1));
    }

    function reload() {
        Bridge.listApps(window.mode);
        Bridge.listCategories(window.mode);
    }

    Connections {
        target: Bridge
        function onAppsReceived(mode, apps) { if (mode === window.mode) page.allApps = apps; }
        function onCategoriesReceived(mode, cats) { if (mode === window.mode) page.categories = cats; }
    }
    Connections { target: window; function onModeChanged() { page.categoryId = 0; page.reload() } }
    Component.onCompleted: reload()

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 14

        Row {
            width: parent.width
            spacing: 16
            Text {
                text: qsTr("All Apps") + " — " + window.mode
                color: ThemeManager.text
                font.pixelSize: 22
                font.bold: true
                anchors.verticalCenter: parent.verticalCenter
            }
            TextField {
                id: search
                width: 260
                placeholderText: qsTr("Filtrer…")
                color: ThemeManager.text
                onTextChanged: page.query = text
            }
        }

        CategoryChips {
            width: parent.width
            categories: page.categories
            selectedId: page.categoryId
            onSelected: (id) => page.categoryId = id
        }

        AppGrid {
            width: parent.width
            height: parent.height - y
            mode: window.mode
            iconSize: window.iconSize
            favoriteIds: window.favoriteIds
            model: page.filtered
            emptyText: Bridge.connected ? qsTr("Aucune application.") : qsTr("Core hors-ligne.")
            onAppActivated: (id, url, isWeb) => window.openApp(id, url, isWeb)
            onFavoriteToggled: (id) => window.toggleFav(id)
        }
    }
}
