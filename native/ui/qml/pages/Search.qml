import QtQuick
import QtQuick.Controls
import MJQbe

// Live filter over the current mode's apps.
Rectangle {
    id: page
    color: ThemeManager.bg

    property string mode: "tv"
    property var allApps: []

    ListModel { id: results }

    Connections {
        target: Bridge
        function onAppsReceived(mode, apps) {
            if (mode !== page.mode)
                return;
            page.allApps = apps;
            page.applyFilter(field.text);
        }
    }

    Component.onCompleted: Bridge.listApps(page.mode)
    onModeChanged: Bridge.listApps(page.mode)

    function applyFilter(query) {
        const needle = query.toLowerCase();
        results.clear();
        for (let i = 0; i < page.allApps.length; ++i) {
            const a = page.allApps[i];
            if (needle.length === 0 || a.name.toLowerCase().indexOf(needle) !== -1)
                results.append(a);
        }
    }

    Column {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 16

        TextField {
            id: field
            width: parent.width
            placeholderText: qsTr("Rechercher une application…")
            color: ThemeManager.text
            onTextChanged: page.applyFilter(text)
        }

        GridView {
            width: parent.width
            height: parent.height - field.height - 16
            cellWidth: 106
            cellHeight: 120
            model: results
            clip: true

            delegate: AppCard {
                required property var model
                appName: model.name
                url: model.url ? model.url : ""
                onActivated: if (url.length > 0) Qt.openUrlExternally(url)
            }
        }
    }
}
