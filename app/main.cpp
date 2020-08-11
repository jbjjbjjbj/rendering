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

    Material blue(Color(0.3, 0.3, 1), 1);
    Material red(Color(1, 0.3, 0.3), 1);
    Material white(Color(1, 1, 1), 1);

    scn.addShape(new Sphere(blue, Vec3d(2, 6, -1), 1));
    scn.addShape(new Sphere(white, Vec3d(0, 4, -1), 1.3));
    scn.addShape(new Sphere(white, Vec3d(-2, 5, -2), 1.3));
    //scn.addShape(new Sphere(Vec3d(0, 7, 0), 0.5));

    scn.addShape(new Plane(white, Vec3d(0, 0, 0), Vec3d(0, 1, 0)));
    scn.addShape(new Plane(white, Vec3d(0, 10, 0), Vec3d(0, 1, 0)));
    scn.addShape(new Plane(white, Vec3d(0, 0, -5), Vec3d(0, 0, 1)));
    scn.addShape(new Plane(red, Vec3d(-5, 0, 0), Vec3d(1, 0, 0)));
    scn.addShape(new Plane(blue, Vec3d(5, 0, 0), Vec3d(1, 0, 0)));

    Renderer render(scn, Vec3d(0, 5, 4), Vec3d(0, 5, 0), 500, 500);

    MainWindow main(render);
    main.show();

    return a.exec();
}
