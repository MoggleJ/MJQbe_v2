cmake_minimum_required(VERSION 3.21)

project(mjqbe-native VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.4 REQUIRED COMPONENTS Core Gui Qml Quick Network)

qt_standard_project_setup()

qt_add_executable(mjqbe-native
    src/main.cpp
    src/NativeBridge.cpp
    src/NativeBridge.h
    src/TerminalController.cpp
    src/TerminalController.h
)

# ThemeManager is a QML singleton (shared palette, 10 themes).
set_source_files_properties(qml/ThemeManager.qml PROPERTIES QT_QML_SINGLETON_TYPE TRUE)

qt_add_qml_module(mjqbe-native
    URI MJQbe
    VERSION 1.0
    RESOURCE_PREFIX "/qt/qml"
    QML_FILES
        qml/Main.qml
        qml/ThemeManager.qml
        qml/Sidebar.qml
        qml/SidebarButton.qml
        qml/Clock.qml
        qml/AppCard.qml
        qml/CategoryChips.qml
        qml/AppGrid.qml
        qml/Gauge.qml
        qml/LoadingCube.qml
        qml/pages/Home.qml
        qml/pages/AllApps.qml
        qml/pages/Search.qml
        qml/pages/Settings.qml
        qml/pages/Login.qml
        qml/pages/Dev.qml
)

target_link_libraries(mjqbe-native PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::Network
)

install(TARGETS mjqbe-native
    RUNTIME DESTINATION bin
)
