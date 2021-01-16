#ifndef VECTOR_H
#define VECTOR_H

#include <iostream>
#include <math.h>

class Vec2d {
    public:
        Vec2d();
        Vec2d(double x, double y);

        void set(double x, double y);

        double m_x, m_y, m_z;
};

class Vec3d {
    public:
        Vec3d();
        Vec3d(double x, double y, double z);

        static inline Vec3d FromSpherical(double sinTheta, double cosTheta, double phi) {
            return Vec3d(sinTheta * std::cos(phi),
                    sinTheta * std::sin(phi),
                    cosTheta);
        }

        void set(double x, double y, double z);
        void normalize();

        double length() const;
        double dot(const Vec3d &vec) const;

        Vec3d cross(const Vec3d &vec) const;

        // Operators
        Vec3d operator+(const Vec3d &vec) const;
        Vec3d& operator+=(const Vec3d &vec);
        Vec3d operator-(const Vec3d &vec) const;
        Vec3d operator-() const;
        Vec3d operator*(double) const;
        Vec3d operator*(const Vec3d &vec) const;
        Vec3d operator/(double) const;

        friend std::ostream& operator<<(std::ostream& os, const Vec3d &v);

        double m_x, m_y, m_z;
};

#endif
