#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QDebug>
#include <QImage>
#include <QMainWindow>

#include "utils/Alert.hpp"
#include "utils/Image.hpp"

QT_BEGIN_NAMESPACE
namespace Ui
{
class MainWindow;
}
QT_END_NAMESPACE

namespace visualization
{

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

public slots:
    void handleImageNotif(const utils::Image &img);
    void handleAlertNotif(const utils::Alert &alert);

private:
    Ui::MainWindow *ui;
};

} // namespace visualization
#endif // MAINWINDOW_H
