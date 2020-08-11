#include "object.hpp"

#include <math.h>
#include <iostream>
#include "common.hpp"

void Color::clamp() {
    if (m_x > 1) { m_x = 1; }
    if (m_y > 1) { m_y = 1; }
    if (m_z > 1) { m_z = 1; }
}

Material::Material(Color color, double defuse) {
    m_color = color;
    m_defuse = defuse;
}

Color Material::reflect(const Vec3d &normal, const Vec3d &in, const Vec3d &out) const {
    return Vec3d(m_color) * (out.dot(normal) * m_defuse);
}

Sphere::Sphere(const Material &mat, Vec3d center, double radius) : Shape(mat) {
    m_center = center;
    m_radius = radius;
}

Plane::Plane(const Material &mat, Vec3d start, Vec3d norm) : Shape(mat) {
    m_start = start;
    m_norm = norm;

    m_norm.normalize();
}

Vec3d Sphere::norm_at(const Vec3d &point, const Vec3d&) const {
    auto res = point - m_center;
    res.normalize();
    return res;
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
double Sphere::intersect(const Ray &ray, bool skip_dist) const {
    // Calculate O - C used multiple places
    auto oc = ray.m_start - m_center;
    // Calculate components of quadratic formula
    // a = 1 when ray.direction is a unit vector
    auto a = 1;
    auto b = 2 * ray.m_direction.dot(oc);
    auto c = oc.dot(oc) - m_radius * m_radius;

    // Solve quadratic function
    auto discr = b * b - 4 * a * c;
    if (discr < 0) {
        // No solution
        return -1;
    }
    if (skip_dist) {
        // Do not calculate distance
        return 1;
    }

    auto q = (b > 0) ?
        -0.5 * (b + sqrt(discr)):
        -0.5 * (b - sqrt(discr));
    auto t1 = q; // Assuming a = 1
    auto t0 = c / q;

    // Find correct result
    if (t0 <= ZERO_APPROX) {
        t0 = t1;
    }

    if (t0 <= ZERO_APPROX) {
        return -1;
    }

    return t0;
}

Vec3d Plane::norm_at(const Vec3d&, const Vec3d &indir) const {
    auto scale = m_norm.dot(indir);
    return scale > 0 ? -m_norm : m_norm;
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection
// Requires that vectors are normalized
// Skip dist is ignored as distance must be calculated
double Plane::intersect(const Ray &ray, bool) const {
    // If ray is parallel
    auto nr = m_norm.dot(ray.m_direction);
    if (abs(nr) < ZERO_APPROX) {
        return -1;
    }

    // Calculate distance
    auto dist = m_norm.dot(m_start - ray.m_start) / nr;
    if (dist < 0) {
        return -1;
    }

    return dist;
}
