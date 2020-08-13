#ifndef RENDER_H
#define RENDER_H

#include "vector.hpp"
#include "ray.hpp"
#include "scene.hpp"

// Samples a random direction in a hemisphere, cosine weighed
// https://blog.thomaspoulet.fr/uniform-sampling-on-unit-hemisphere/
class Sampler {
    public:
        Sampler();
        void seed(unsigned seed);

        Vec3d sample(const Vec3d &norm);

    private:
        double random();
        unsigned m_seed;
};

class Renderer {
    public:
    Renderer(const Scene &scn, Vec3d eye, Vec3d target, unsigned width, unsigned height, unsigned maxhops);

    Color render(unsigned x, unsigned y, unsigned samples);

    unsigned m_width, m_height;
    Sampler m_sampler;

    private:
    void recalculate();
    Ray findray(double x, double y) const ;

    Color pathtrace_sample(const Ray &r, unsigned hop);
    // Will return first result less than chk_dist. 
    // This is ignored if chk_dist is 0.
    // If dist is non-null the resulting distance is written here.
    const Shape* cast_ray(const Ray &r, double chk_dist, double *dist);

    const Scene &m_scn;

    // User options
    Vec3d m_eye, m_target;
    unsigned m_maxhops;

    // Calculated values
    Vec3d m_qx, m_qy, m_blc;

};

#endif
