#include "render.hpp"
#include "vector.hpp"
#include "common.hpp"

#include <cstdlib>
#include <math.h>
#include <iostream>

#define FOV 1.74533

// Uniform sampling
#define SAMPLING_POWER 0

const Vec3d up = Vec3d(0, 1, 0);

void Random::seed(unsigned seed) {
    for (unsigned i = 0; i < seed; i++) {
        rand_r(&m_seed);
    }
}

double Random::operator()() {
    return (double)rand_r(&m_seed) / (double)RAND_MAX;
}

Sampler::Sampler(Random &src) : m_src(src) { }

Vec3d Sampler::sample(const Vec3d &norm) {
    /*
    auto theta = asin(pow(1 - random(), (double)1 / (1 + SAMPLING_POWER)));
    auto phi = 2 * M_PI * random();
    */

    auto theta = 2.0 * M_PI * m_src();
    auto phi = acos(2.0 * m_src() - 1.0);

    auto sinphi = sin(phi);

    auto newvec = Vec3d(cos(theta) * sinphi, sin(theta) * sinphi, cos(phi));

    if (newvec.dot(norm) <= 0) {
        newvec = -newvec;
    }

    return newvec;
}

Renderer::Renderer(const Scene &scn, Vec3d eye, Vec3d target, unsigned width, unsigned height, unsigned maxhops) : 
    m_sampler(m_random),
    m_scn(scn)
{
    m_eye = eye;
    m_target = target;
    m_width = width;
    m_height = height;
    m_maxhops = maxhops;

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

Color Renderer::render(unsigned x, unsigned y, unsigned samples) {

    Color sum(0, 0, 0);

    for (unsigned i = 0; i < samples; i++) {
        auto r = findray(x + m_random(), y + m_random());
        sum += pathtrace_sample(r, 0);
    }

    if (samples < 2) {
        return sum;
    } else {
        return Vec3d(sum) / (double)samples;
    }
}

Color Renderer::pathtrace_sample(const Ray &r, unsigned hop) {
    if (hop >= m_maxhops) {
        return Color(0, 0, 0);
    }

    double dist;
    auto res = cast_ray(r, 0, &dist);

    if (!res) {
        return Color(0, 0, 0);
    }

    auto col = res->m_mat.emits();
    if (res->m_mat.reflects()) {
        // Calculate endpoint
        auto end = r.m_start + r.m_direction * dist;
        auto norm = res->norm_at(end, r.m_direction);

        auto randdir = m_sampler.sample(norm);
        auto newray = Ray(end, randdir, true);
        auto incol = pathtrace_sample(newray, hop+1);

        col += res->m_mat.reflect(norm, r.m_direction, newray.m_direction, incol);
    }

    return col;
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
