#ifndef VECTOR_H
#define VECTOR_H

class Vec3d {
    Vec3d();
    Vec3d(double x, double y, double z);

    void set(double x, double y, double z);
    void normalize();

    double length();

    Vec3d cross(const Vec3d &vec);

    // Operators
};

#endif
