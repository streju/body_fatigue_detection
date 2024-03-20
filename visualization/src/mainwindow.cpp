#include "mainwindow.h"
#include "./ui_mainwindow.h"

namespace visualization
{

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::handleImageNotif(const utils::Image &img)
{
    qDebug() << "MainWindow received image.";
    if (img.type == utils::ImgType::Raw)
    {
        ui->raw_img_label->setPixmap(QPixmap::fromImage(img.data, Qt::AutoColor));
        ui->raw_img_label->setFixedSize(img.data.size());
    }
    else if (img.type == utils::ImgType::DebugLandmarks)
    {
        ui->debug_img_label->setPixmap(QPixmap::fromImage(img.data, Qt::AutoColor));
        ui->debug_img_label->setFixedSize(img.data.size());
    }
}

void MainWindow::handleAlertNotif(const utils::Alert &alert)
{
    qDebug() << "MainWindow received alert: " << alert.msg << ", action: " << (alert.isStart ? "start" : "stop");

    if (alert.isStart)
    {
        if (ui->active_alerts_list->findItems(alert.msg, Qt::MatchStartsWith).empty())
        {
            ui->active_alerts_list->addItem(alert.msg);
        }
    }
    else
    {
        if (auto it = ui->active_alerts_list->findItems(alert.msg, Qt::MatchStartsWith).first())
        {
            delete it;
        }
        // TODO: error should not happen
    }
}

} // namespace visualization