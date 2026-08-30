import QtQuick
import QtQuick.Controls
import MJQbe

// Root window: fixed 250px sidebar + StackView content area.
ApplicationWindow {
    id: window
    visible: true
    width: 1280
    height: 800
    title: qsTr("MJQbe")
    color: ThemeManager.bg

    readonly property var modeTitles: ({ "tv": "MJ TV", "desktop": "MJ Desktop", "dev": "MJ Dev" })
    property string mode: "tv"

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
    Component { id: allAppsPage;  AllApps { mode: window.mode } }
    Component { id: searchPage;   Search { mode: window.mode } }
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
        // Fullscreen on the Pi; --windowed for desktop dev.
        if (typeof startFullScreen !== "undefined" && startFullScreen)
            window.showFullScreen();
        else
            window.showNormal();
        Bridge.ping();
    }
}
