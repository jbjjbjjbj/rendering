#include <iostream>
#include <qapplication.h>
#include <qpushbutton.h>

#include "mainwindow.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    MainWindow main;
    main.show();

    return a.exec();
}
