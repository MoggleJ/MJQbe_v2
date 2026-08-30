import QtQuick

// Live clock for the bottom of the sidebar (Timer, 1 s tick).
Text {
    id: clock

    function refresh() {
        clock.text = Qt.formatDateTime(new Date(), "ddd dd MMM  HH:mm:ss");
    }

    font.pixelSize: 13
    Component.onCompleted: refresh()

    Timer {
        interval: 1000
        running: true
        repeat: true
        onTriggered: clock.refresh()
    }
}
