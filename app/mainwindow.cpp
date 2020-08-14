#include "mainwindow.hpp"
#include <qaction.h>
#include <qapplication.h>
#include <qlabel.h>
#include <qnamespace.h>
#include <QFileDialog>
#include <QMessageBox>

MainWindow::MainWindow(Renderer r)
    : m_drawer(500, 500),
    runstatus("Not running", this),
    m_render(this, m_drawer, r, &runstatus)
{

    setCentralWidget(&m_drawer);

    auto saveAct = new QAction(tr("&Save as"), this);
    saveAct->setStatusTip(tr("Save the rendered image"));
    connect(saveAct, &QAction::triggered, this, &MainWindow::saveimage);

    auto stopAct = new QAction(tr("&Stop"), this);
    stopAct->setStatusTip(tr("Stop and sync threads"));
    QObject::connect(stopAct, &QAction::triggered, &m_render, &RenderCoordinator::stop);

    fileMenu = menuBar()->addMenu(tr("&File"));
    fileMenu->addAction(saveAct);

    fileMenu = menuBar()->addMenu(tr("&Render"));
    fileMenu->addAction(stopAct);

    helpMenu = menuBar()->addMenu(tr("&Help"));
    helpMenu->addAction(tr("About Qt"), qApp, &QApplication::aboutQt);

    statusBar()->addWidget(&runstatus);
}

void MainWindow::saveimage() {

    QGuiApplication::setOverrideCursor(Qt::WaitCursor);
    QString fileName = QFileDialog::getSaveFileName(this,
            tr("Save image"), "", tr("PNG image (*.png);;All Files (*)"));
    if (fileName.isEmpty()) {
        return;
    }

    if (!m_drawer.m_img.save(fileName)) {
        QMessageBox::information(this, tr("Unable to save file"), "");
    }
}
