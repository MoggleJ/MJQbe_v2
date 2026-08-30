import QtQuick
import QtQuick.Controls
import MJQbe

// Local admin authentication (bcrypt verify in the core). Used as the re-auth
// gate for Dev mode from Sprint 5 on.
Rectangle {
    id: page
    color: ThemeManager.bg

    property string message: ""

    Connections {
        target: Bridge
        function onLoginResult(ok, role, error) {
            page.message = ok ? (qsTr("Connecté — rôle : ") + role)
                              : (qsTr("Échec : ") + error);
        }
    }

    Column {
        anchors.centerIn: parent
        spacing: 12
        width: 280

        Text {
            text: qsTr("Authentification admin")
            color: ThemeManager.text
            font.pixelSize: 20
            font.bold: true
        }

        TextField {
            id: user
            width: parent.width
            placeholderText: qsTr("Utilisateur")
            color: ThemeManager.text
        }

        TextField {
            id: pass
            width: parent.width
            placeholderText: qsTr("Mot de passe")
            echoMode: TextInput.Password
            color: ThemeManager.text
            onAccepted: Bridge.login(user.text, pass.text)
        }

        Button {
            text: qsTr("Se connecter")
            onClicked: Bridge.login(user.text, pass.text)
        }

        Text {
            width: parent.width
            text: page.message
            color: ThemeManager.textDim
            font.pixelSize: 12
            wrapMode: Text.Wrap
        }
    }
}
