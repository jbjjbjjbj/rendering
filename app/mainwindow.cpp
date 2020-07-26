#include "mainwindow.hpp"

MainWindow::MainWindow()
    : drawer(500, 500) {

    setCentralWidget(&drawer);

}
