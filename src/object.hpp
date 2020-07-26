#ifndef OBJECT_H
#define OBJECT_H

#include <memory>
#include "vector.hpp"
#include "ray.hpp"

class Material {
};

class Object {
    public:
        void setMaterial(std::shared_ptr<Material> m);

        std::shared_ptr<Material> m_mat;

        virtual Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const = 0;
        virtual double intersect(const Ray &ray, bool skip_dist) const = 0;
};

class Sphere : Object {
    public:
        Sphere(Vec3d center, double radius);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_center;
        double m_radius;
};

class Plane : Object {
    public:
        Plane(Vec3d start, Vec3d norm);
        Vec3d norm_at(const Vec3d &point, const Vec3d &indir) const;
        double intersect(const Ray &ray, bool skip_dist) const;

    private:
        Vec3d m_start;
        Vec3d m_norm;
};

#endif
