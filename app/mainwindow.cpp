#include "mainwindow.hpp"

MainWindow::MainWindow(Renderer r)
    : m_drawer(500, 500),
    m_render(this, m_drawer, r)
{

    setCentralWidget(&m_drawer);



}
