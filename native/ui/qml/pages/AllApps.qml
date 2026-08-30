import QtQuick
import MJQbe

// Grid of apps for the current mode, fed by mjqbe-core (apps.list).
Rectangle {
    id: page
    color: ThemeManager.bg

    property string mode: "tv"

    ListModel { id: appsModel }

    Connections {
        target: Bridge
        function onAppsReceived(mode, apps) {
            if (mode !== page.mode)
                return;
            appsModel.clear();
            for (let i = 0; i < apps.length; ++i)
                appsModel.append(apps[i]);
        }
    }

    Component.onCompleted: Bridge.listApps(page.mode)
    onModeChanged: Bridge.listApps(page.mode)

    Text {
        id: header
        anchors { top: parent.top; left: parent.left; margins: 24 }
        text: qsTr("All Apps") + " — " + page.mode
        color: ThemeManager.text
        font.pixelSize: 22
        font.bold: true
    }

    GridView {
        id: grid
        anchors {
            top: header.bottom; left: parent.left; right: parent.right; bottom: parent.bottom
            topMargin: 16; leftMargin: 24; rightMargin: 24; bottomMargin: 24
        }
        cellWidth: 106
        cellHeight: 120
        model: appsModel
        clip: true

        delegate: AppCard {
            required property var model
            appName: model.name
            iconName: model.icon ? model.icon : ""
            url: model.url ? model.url : ""
            onActivated: if (url.length > 0) Qt.openUrlExternally(url)
        }

        Text {
            anchors.centerIn: parent
            visible: appsModel.count === 0
            text: Bridge.connected ? qsTr("Aucune application.") : qsTr("Core hors-ligne.")
            color: ThemeManager.textDim
        }
    }
}
