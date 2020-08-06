#ifndef MAIN_H
#define MAIN_H

#include <QMainWindow>

#include "draw.hpp"
#include "rendercoord.hpp"
#include <render.hpp>

class MainWindow : public QMainWindow {
    Q_OBJECT

    public:
        MainWindow(Renderer r);

    private:
        DrawWidget m_drawer;
        RenderCoordinator m_render;
};

#endif
