#include "mainwindow.h"
#include "./ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    connect(ui->pushButton, SIGNAL(clicked()), this, SLOT(test_func()));
}


MainWindow::~MainWindow()
{
    delete ui;
}


void MainWindow::test_func()
{
    ui->label->setText("fefzefz");
}
