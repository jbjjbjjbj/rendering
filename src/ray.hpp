#ifndef RAY_H
#define RAY_H

#include "vector.hpp"

class Ray {
    public:
        Ray(Vec3d start, Vec3d direction, bool normalize);
        Ray(Vec3d a, Vec3d b);

        Vec3d m_start;
        Vec3d m_direction;
};

#endif
