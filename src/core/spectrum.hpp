#ifndef SPECTRUM_H
#define SPECTRUM_H

#include "core/common.hpp"

// Contains a RGB spectrum value
class Spectrum {
public:
    Spectrum(double v = 0);
    static Spectrum FromRGB(double r, double g, double b);

    Spectrum &operator+=(const Spectrum &o);
    Spectrum &operator*=(double o);
    Spectrum operator+(const Spectrum &o) const;
    Spectrum operator-(const Spectrum &o) const;
    Spectrum operator*(const Spectrum &o) const;
    Spectrum operator/(const Spectrum &o) const;

    Spectrum clamp(double low = 0, double high = INFINITY) const;

    double R() const { return c[0]; }
    double G() const { return c[1]; }
    double B() const { return c[2]; }

private:
    double c[3];
};

#endif
