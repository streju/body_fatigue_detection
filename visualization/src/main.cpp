#include <iostream>
#include <string>

#include <QApplication>
#include <QDebug>

#include "ConfigParser.hpp"
#include "mainwindow.h"
#include "middleware/grpc/ImageListener.hpp"

using namespace visualization;

int main(int argc, char *argv[])
{
    std::string configFile;
    if (argc < 2)
    {
        configFile = "config.json";
        qInfo() << "No config file passed as argument. Trying with default: " << configFile.c_str();
    }
    else
    {
        configFile = argv[1];
        qInfo() << "Passed config file: " << configFile.c_str();
    }
    ConfigParser parser;
    const auto parseResult = parser.parseJsonFile(configFile);
    if (const auto err = parseResult.getErr())
    {
        qCritical() << err->c_str();
        return 1;
    }

    middleware::ImageListener listener{parser.getVisualizationServerFullAddr()};

    QApplication app(argc, argv);
    MainWindow mainWindow;

    QObject::connect(
        &listener, &middleware::ImageListener::CameraFrameNotif, &mainWindow, &MainWindow::handleImageNotif);

    mainWindow.show();
    listener.start();

    return app.exec();
}
