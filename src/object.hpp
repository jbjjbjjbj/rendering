#ifndef OBJECT_H
#define OBJECT_H

#include <memory>
#include "vector.hpp"
#include "ray.hpp"

class Color : public Vec3d {
    public:
        Color() {}
        Color(const Vec3d &v) : Vec3d(v) {}
        Color(double r, double g, double b) : Vec3d(r, g, b) {}

        uint8_t r() { return m_x * 255; }
        uint8_t g() { return m_y * 255; }
        uint8_t b() { return m_z * 255; }

        void clamp();

};

// Implements phong BRDF 
class Material {
    public:
        Material(Color color, double defuse);

        Color reflect(const Vec3d &normal, const Vec3d &in, const Vec3d &out) const;
    private:
        Color m_color;
        double m_defuse;
};

class Shape {
    public:
        Shape(const Material &mat) : m_mat(mat) { }

        virtual Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const = 0;
        virtual double intersect(const Ray &ray, bool skip_dist) const = 0;

        const Material &m_mat;
};

class Sphere : public Shape {
    public:
        Sphere(const Material &mat, Vec3d center, double radius);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_center;
        double m_radius;
};

class Plane : public Shape {
    public:
        Plane(const Material &mat, Vec3d start, Vec3d norm);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_start;
        Vec3d m_norm;
};

#endif
