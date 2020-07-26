#include "ray.hpp"

Ray::Ray(Vec3d start, Vec3d direction, bool normalize) {
    m_start = start;
    m_direction = direction;

    if (normalize) {
        m_direction.normalize();
    }
}

Ray::Ray(Vec3d a, Vec3d b) {
    m_start = a;
    m_direction = b - a;

    m_direction.normalize();
}
