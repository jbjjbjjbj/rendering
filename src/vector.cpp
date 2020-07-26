#include "vector.hpp"

#include <math.h>

Vec3d::Vec3d() {
    set(0, 0, 0);
}

Vec3d::Vec3d(double x, double y, double z) {
    set(x, y, z);
}

void Vec3d::set(double x, double y, double z) {
    m_x = x;
    m_y = y;
    m_z = z;
}

void Vec3d::normalize() {
    auto len = length();
    if (len == 0) {
        throw "Normalizing zero vector";
    }

    m_x /= len;
    m_y /= len;
    m_z /= len;
}

double Vec3d::length() const {
    return sqrt(m_x * m_x + m_y * m_y + m_z * m_z);
}

double Vec3d::dot(const Vec3d &vec) const {
    return m_x * vec.m_x + m_y * vec.m_y + m_z * vec.m_z;
}

Vec3d Vec3d::operator-(const Vec3d &vec) const {
    return Vec3d(
            m_x - vec.m_x, 
            m_y - vec.m_y,
            m_z - vec.m_z
            );
}

Vec3d Vec3d::operator-() const {
    return Vec3d(
            -m_x,
            -m_y,
            -m_z
            );
}

