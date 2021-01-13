#ifndef RENDER_H
#define RENDER_H

#include "core/vector.hpp"
#include "core/ray.hpp"
#include "scene.hpp"

class Random {
    public:
        void seed(unsigned seed);
        double operator()();

    private:
        unsigned m_seed;
};

// Samples a random direction in a hemisphere, cosine weighed
// https://blog.thomaspoulet.fr/uniform-sampling-on-unit-hemisphere/
class Sampler {
    public:
        Sampler(Random &src);

        Vec3d sample(const Vec3d &norm);

    private:
        Random &m_src;
};

class Renderer {
    public:
    Renderer(const Scene &scn, Vec3d eye, Vec3d target, unsigned width, unsigned height, unsigned maxhops);

    Spectrum render(unsigned x, unsigned y, unsigned samples);

    unsigned m_width, m_height;
    Sampler m_sampler;

    private:
    void recalculate();
    Ray findray(double x, double y) const ;

    Spectrum pathtrace_sample(const Ray &r, unsigned hop);
    // Will return first result less than chk_dist. 
    // This is ignored if chk_dist is 0.
    // If dist is non-null the resulting distance is written here.
    const Shape* cast_ray(const Ray &r, double chk_dist, double *dist);

    const Scene &m_scn;

    Random m_random;

    // User options
    Vec3d m_eye, m_target;
    unsigned m_maxhops;

    // Calculated values
    Vec3d m_qx, m_qy, m_blc;

};

#endif
