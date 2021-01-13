#include "spectrum.hpp"

static double clamp(double v, double low = 0, double high = 0) {
    if (v < low) {
        return low;
    } 
    if (v > high) {
        return high;
    }
    return v;
}

Spectrum::Spectrum(double v) {
    c[0] = v;
    c[1] = v;
    c[2] = v;
}

Spectrum Spectrum::FromRGB(double r, double g, double b) {
    Spectrum ret;
    ret.c[0] = r;
    ret.c[1] = g;
    ret.c[2] = b;

    return ret;
}

Spectrum &Spectrum::operator+=(const Spectrum &o) {
    c[0] += o.c[0];
    c[1] += o.c[1];
    c[2] += o.c[2];

    return *this;
}

Spectrum &Spectrum::operator*=(double o) {
    c[0] *= o;
    c[1] *= o;
    c[2] *= o;

    return *this;
}

Spectrum Spectrum::operator+(const Spectrum &o) const {
    Spectrum ret = *this;

    ret.c[0] += o.c[0];
    ret.c[1] += o.c[1];
    ret.c[2] += o.c[2];

    return ret;
}

Spectrum Spectrum::operator-(const Spectrum &o) const {
    Spectrum ret = *this;

    ret.c[0] -= o.c[0];
    ret.c[1] -= o.c[1];
    ret.c[2] -= o.c[2];

    return ret;
}

Spectrum Spectrum::operator*(const Spectrum &o) const {
    Spectrum ret = *this;

    ret.c[0] *= o.c[0];
    ret.c[1] *= o.c[1];
    ret.c[2] *= o.c[2];

    return ret;
}

Spectrum Spectrum::operator/(const Spectrum &o) const {
    Spectrum ret = *this;

    ret.c[0] /= o.c[0];
    ret.c[1] /= o.c[1];
    ret.c[2] /= o.c[2];

    return ret;
}

Spectrum Spectrum::clamp(double low, double high) const {
    Spectrum ret;
    ret.c[0] = ::clamp(c[0], low, high);
    ret.c[1] = ::clamp(c[1], low, high);
    ret.c[2] = ::clamp(c[2], low, high);
    return ret;
}
