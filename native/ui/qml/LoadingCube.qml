import QtQuick
import MJQbe

// MJQbe loading cube — 4 side faces rotating about the Y axis (GPU-composited
// via Item.transform). Deliberately cheap: no Qt3D, no ShaderEffect, just
// animated perspective transforms so it stays well under budget on the Pi.
Item {
    id: root
    property real size: 96
    implicitWidth: size
    implicitHeight: size

    // Continuous spin driver.
    property real spin: 0
    NumberAnimation on spin {
        from: 0; to: 360
        duration: 2400
        loops: Animation.Infinite
        running: root.visible
    }

    Item {
        id: stage
        anchors.centerIn: parent
        width: root.size
        height: root.size

        Repeater {
            model: 4
            delegate: Rectangle {
                required property int index
                anchors.centerIn: parent
                width: root.size
                height: root.size
                radius: root.size * 0.12
                color: index % 2 === 0 ? ThemeManager.accent : ThemeManager.surface
                border.color: ThemeManager.border
                opacity: 0.92

                transform: [
                    Rotation {
                        origin.x: root.size / 2
                        origin.y: root.size / 2
                        axis { x: 0; y: 1; z: 0 }
                        angle: root.spin + index * 90
                    },
                    Translate { x: 0 } // face pushed out by the perspective below
                ]
            }
        }
        // Fake perspective: scale the whole stage subtly with the spin.
        transform: Scale {
            origin.x: root.size / 2
            origin.y: root.size / 2
            xScale: 0.82 + 0.18 * Math.abs(Math.cos(root.spin * Math.PI / 180))
        }
    }

    Text {
        anchors { horizontalCenter: parent.horizontalCenter; top: stage.bottom; topMargin: 16 }
        text: "MJQbe"
        color: ThemeManager.textDim
        font.pixelSize: 14
        font.bold: true
    }
}
