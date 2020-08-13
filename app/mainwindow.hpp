#ifndef MAIN_H
#define MAIN_H

#include <QMainWindow>
#include <QLabel>
#include <QMenuBar>
#include <QStatusBar>

#include "draw.hpp"
#include "rendercoord.hpp"
#include <qmainwindow.h>
#include <render.hpp>

class MainWindow : public QMainWindow {
    Q_OBJECT

    public:
        MainWindow(Renderer r);

    private slots:
        void saveimage();
    private:
        DrawWidget m_drawer;
        QLabel runstatus;
        RenderCoordinator m_render;

        QMenu *fileMenu;
        QMenu *helpMenu;
};

#endif
