import QtQuick
import QtQuick.Controls
import MJQbe

// Root window: fixed 250px sidebar + StackView content area.
// Owns cross-page state: current user session, settings (theme/layout/icon_size),
// and the favourites set.
ApplicationWindow {
    id: window
    visible: true
    width: 1280
    height: 800
    title: qsTr("MJQbe")
    color: ThemeManager.bg

    readonly property var modeTitles: ({ "tv": "MJ TV", "desktop": "MJ Desktop", "dev": "MJ Dev" })
    property string mode: "tv"
    property string layout: "grid"
    property string iconSize: "medium"
    property var favoriteIds: []

    // --- core wiring -------------------------------------------------------
    Connections {
        target: Bridge
        function onSessionChanged() {
            Bridge.getSettings();
            Bridge.listFavorites();
        }
        function onSettingsReceived(s) {
            if (s.theme) ThemeManager.setTheme(s.theme);
            if (s.layout) window.layout = s.layout;
            if (s.icon_size) window.iconSize = s.icon_size;
            if (s.default_mode && window.mode !== "dev") window.mode = s.default_mode;
        }
        function onFavoritesReceived(ids) { window.favoriteIds = ids; }
        function onFavoriteToggled(appId, favorited) {
            var next = window.favoriteIds.slice();
            var i = next.indexOf(appId);
            if (favorited && i === -1) next.push(appId);
            else if (!favorited && i !== -1) next.splice(i, 1);
            window.favoriteIds = next;
        }
    }

    function openApp(appId, url, isWeb) {
        if (url && url.length > 0)
            Qt.openUrlExternally(url);
        // Embedded (isWeb === false) apps: handled in a later sprint (QWebEngineView).
    }
    function toggleFav(appId) { Bridge.toggleFavorite(appId); }
    function isFav(appId) { return window.favoriteIds.indexOf(appId) !== -1; }

    // --- layout ----------------------------------------------------------
    Row {
        anchors.fill: parent

        Sidebar {
            id: sidebar
            width: 250
            height: parent.height
            modeTitle: window.modeTitles[window.mode]
            currentPage: stack.currentPageName
            onNavigate: (page) => window.go(page)
            onSwitchMode: window.toggleMode()
            onOpenSettings: window.go("Settings")
        }

        StackView {
            id: stack
            width: parent.width - sidebar.width
            height: parent.height
            property string currentPageName: "Home"
            initialItem: homePage
        }
    }

    Component { id: homePage;     Home {} }
    Component { id: allAppsPage;  AllApps {} }
    Component { id: searchPage;   Search {} }
    Component { id: settingsPage; Settings {} }
    Component { id: loginPage;    Login {} }

    function go(page) {
        const map = {
            "Home": homePage, "AllApps": allAppsPage, "Search": searchPage,
            "Settings": settingsPage, "Login": loginPage
        };
        if (map[page] === undefined)
            return;
        stack.currentPageName = page;
        stack.replace(map[page]);
    }

    function toggleMode() {
        window.mode = (window.mode === "tv") ? "desktop" : "tv";
        window.go(stack.currentPageName); // rebuild the data-bound page for the new mode
    }

    Component.onCompleted: {
        if (typeof startFullScreen !== "undefined" && startFullScreen)
            window.showFullScreen();
        else
            window.showNormal();
        Bridge.ping();
        Bridge.fetchSession();
    }
}
