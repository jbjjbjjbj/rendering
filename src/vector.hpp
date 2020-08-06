#ifndef VECTOR_H
#define VECTOR_H

class Vec3d {
    public:
        Vec3d();
        Vec3d(double x, double y, double z);

        void set(double x, double y, double z);
        void normalize();

        double length() const;
        double dot(const Vec3d &vec) const;

        Vec3d cross(const Vec3d &vec) const;

        // Operators
        Vec3d operator+(const Vec3d &vec) const;
        Vec3d operator-(const Vec3d &vec) const;
        Vec3d operator-() const;
        Vec3d operator*(double) const;

        double m_x, m_y, m_z;
};

#endif
