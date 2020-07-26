#ifndef MAIN_H
#define MAIN_H

#include <QMainWindow>

#include "draw.hpp"

class MainWindow : public QMainWindow {
    Q_OBJECT

    public:
        MainWindow();

    private:
        DrawWidget drawer;

};

#endif
