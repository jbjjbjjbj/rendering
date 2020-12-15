#include <iostream>
#include <qapplication.h>
#include <qpushbutton.h>

#include "mainwindow.hpp"
#include "vector.hpp"
#include <scene.hpp>
#include <render.hpp>
#include <object.hpp>

using namespace std;

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    Scene scn;
    Config conf;
    conf.m_width = 500;
    conf.m_height = 500;
    conf.m_maxhops = 5;
    conf.m_samples = 1000;
    conf.m_framerate = 3;
    conf.m_workers = 8;


    Material blue(Color(0.3, 0.3, 1), 1);
    Material red(Color(1, 0.3, 0.3), 1);
    Material white(Color(1, 1, 1), 1);
    Material em(Color(1, 1, 1), 0, 2);

    scn.addShape(new Sphere(red, Vec3d(2, 6, -1), 1));
    scn.addShape(new Sphere(white, Vec3d(0, 4, -1), 1.3));
    scn.addShape(new Sphere(white, Vec3d(-2, 5, -2), 1.3));
    scn.addShape(new Sphere(blue, Vec3d(0, 7, 0), 0.5));

    scn.addShape(new Plane(em, Vec3d(0, 0, 0), Vec3d(0, 1, 0)));
    scn.addShape(new Plane(white, Vec3d(0, 10, 0), Vec3d(0, 1, 0)));
    scn.addShape(new Plane(white, Vec3d(0, 0, -5), Vec3d(0, 0, 1)));
    scn.addShape(new Plane(red, Vec3d(-5, 0, 0), Vec3d(1, 0, 0)));
    scn.addShape(new Plane(blue, Vec3d(5, 0, 0), Vec3d(1, 0, 0)));

    RendererConf render(scn, Vec3d(0, 5, 4), Vec3d(0, 5, 0), conf);

    MainWindow main(render, conf);
    main.show();

    return a.exec();
}
