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

            // "Cube up" page transition (PowerPoint-style, CDC §4.4): the leaving
            // page slides up and shrinks away, the entering page rises from below.
            // layer.enabled batches each page into one GPU texture so the move
            // stays cheap on the Pi; it is turned back off once settled.
            replaceExit: Transition {
                PropertyAction { property: "layer.enabled"; value: true }
                SequentialAnimation {
                    ParallelAnimation {
                        NumberAnimation { property: "y"; from: 0; to: -height * 0.6; duration: 200; easing.type: Easing.InCubic }
                        NumberAnimation { property: "opacity"; from: 1; to: 0; duration: 200 }
                        NumberAnimation { property: "scale"; from: 1; to: 0.92; duration: 200 }
                    }
                    PropertyAction { property: "layer.enabled"; value: false }
                    PropertyAction { property: "opacity"; value: 1 }
                    PropertyAction { property: "scale"; value: 1 }
                }
            }
            replaceEnter: Transition {
                PropertyAction { property: "layer.enabled"; value: true }
                SequentialAnimation {
                    ParallelAnimation {
                        NumberAnimation { property: "y"; from: height * 0.6; to: 0; duration: 260; easing.type: Easing.OutCubic }
                        NumberAnimation { property: "opacity"; from: 0; to: 1; duration: 260 }
                        NumberAnimation { property: "scale"; from: 0.92; to: 1; duration: 260 }
                    }
                    PropertyAction { property: "layer.enabled"; value: false }
                }
            }
        }
    }

    // Loading overlay: MJQbe cube until the core connects (or after 4s give up).
    Rectangle {
        id: loadingOverlay
        anchors.fill: parent
        color: ThemeManager.bg
        visible: opacity > 0
        opacity: Bridge.connected ? 0 : 1
        Behavior on opacity { NumberAnimation { duration: 400 } }
        z: 100

        LoadingCube { anchors.centerIn: parent; size: 120 }

        Timer {
            interval: 4000; running: true; repeat: false
            onTriggered: if (!Bridge.connected) loadingOverlay.opacity = 0
        }
        // Swallow input while shown.
        MouseArea { anchors.fill: parent; enabled: parent.opacity > 0.5 }
    }

    Component { id: homePage;     Home {} }
    Component { id: allAppsPage;  AllApps {} }
    Component { id: searchPage;   Search {} }
    Component { id: settingsPage; Settings {} }
    Component { id: loginPage;    Login {} }
    Component { id: devPage;      Dev {} }

    property string previousUserMode: "tv"

    function go(page) {
        const map = {
            "Home": homePage, "AllApps": allAppsPage, "Search": searchPage,
            "Settings": settingsPage, "Login": loginPage, "Dev": devPage
        };
        if (map[page] === undefined)
            return;
        // Dev mode drives the sidebar title; remember the consumption mode to restore.
        if (page === "Dev" && window.mode !== "dev") {
            window.previousUserMode = window.mode;
            window.mode = "dev";
        } else if (page !== "Dev" && window.mode === "dev") {
            window.mode = window.previousUserMode;
        }
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
