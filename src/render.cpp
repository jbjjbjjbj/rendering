#include "render.hpp"
#include "vector.hpp"
#include "common.hpp"

#include <math.h>
#include <iostream>

#define FOV 1.74533

const Vec3d up = Vec3d(0, 1, 0);

Renderer::Renderer(const Scene &scn, Vec3d eye, Vec3d target, unsigned width, unsigned height) : 
    m_scn(scn)
{
    m_eye = eye;
    m_target = target;
    m_width = width;
    m_height = height;

    recalculate();
}

void Renderer::recalculate() {
    auto tmp = m_target - m_eye;

    // Orthogonal vector to E
    auto b = up.cross(tmp);

    b.normalize();
    tmp.normalize();

    auto v = tmp.cross(b);

    // Calculate size of viewplane
    double gx = tan( FOV / 2);
    double gy = gx * ((double) m_height / m_width);

    // Calculate scaling vectors
    m_qx = b * ((2 * gx) / (m_width - 1));
    m_qy = v * ((2 * gy) / (m_height - 1));

    // Calculate starting point
    m_blc = tmp - (b * gx) - (v * gy);
}

Ray Renderer::findray(double x, double y) const {
    auto dir = m_blc + (m_qx * x) + (m_qy * y);
    return Ray(m_eye, dir, true);
}

Color Renderer::render(unsigned x, unsigned y) {
    auto r = findray(x, y);
    return pathtrace_sample(r, 0);
}

Color Renderer::pathtrace_sample(const Ray &r, unsigned hop) {
    double dist;
    auto res = cast_ray(r, 0, &dist);

    if (!res) {
        return Color(0, 0, 0);
    }

    // Calculate endpoint
    auto end = r.m_start + r.m_direction * dist;

    auto norm = res->norm_at(end, r.m_direction);

    // Simulate single light
    auto l = Vec3d(0, 7, 0);

    auto tolight = l - end;
    auto distance = tolight.length();
    tolight.normalize();
    auto lightray = Ray(end, tolight, false);

    if (cast_ray(lightray, distance, nullptr)) {
        return Color(0, 0, 0);
    }

    return res->m_mat.reflect(norm, r.m_direction, tolight);
}

const Shape* Renderer::cast_ray(const Ray &r, double chk_dist, double *dist_ret) {
    const Shape *smallest = nullptr;
    double dist = 0;

    for (auto obj : m_scn.objs) {
        if (!obj) {
            continue;
        }
        auto d = obj->intersect(r, false);
        if (d > ZERO_APPROX) {
            if (chk_dist > 0 && d < chk_dist) {
                dist = d; smallest = obj;
                goto exit;
            }
            if (d < dist || smallest == nullptr) {
                dist = d; smallest = obj;
            }
        }
    }

    if (chk_dist > 0) {
        // If we reach this it means none of the
        // object where within distance.
        return nullptr;
    }

exit:

    if (dist_ret) {
        *dist_ret = dist;
    }

    return smallest;
}
