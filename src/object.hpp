#ifndef OBJECT_H
#define OBJECT_H

#include <memory>
#include "vector.hpp"
#include "ray.hpp"

class Material {
};

class Shape {
    public:
        void setMaterial(std::shared_ptr<Material> m);

        virtual Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const = 0;
        virtual double intersect(const Ray &ray, bool skip_dist) const = 0;
};

class Sphere : public Shape {
    public:
        Sphere(Vec3d center, double radius);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_center;
        double m_radius;
};

class Plane : public Shape {
    public:
        Plane(Vec3d start, Vec3d norm);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_start;
        Vec3d m_norm;
};

#endif
