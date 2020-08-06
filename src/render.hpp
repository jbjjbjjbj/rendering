#ifndef RENDER_H
#define RENDER_H

#include "vector.hpp"
#include "ray.hpp"
#include "scene.hpp"

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

class Renderer {
    public:
    Renderer(const Scene &scn, Vec3d eye, Vec3d target, unsigned width, unsigned height);

    Color render(unsigned x, unsigned y);

    unsigned m_width, m_height;

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

    // Calculated values
    Vec3d m_qx, m_qy, m_blc;

};

#endif
