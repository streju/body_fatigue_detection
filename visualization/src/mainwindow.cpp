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

} // namespace visualization