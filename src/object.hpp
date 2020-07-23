#ifndef OBJECT_H
#define OBJECT_H

#include <memory>
#include "vector.hpp"

class Material {
    Vec3d color;

    double defuse;
    double emissive;
};

class Object {
    public:
        std::shared_ptr<Material> m;
};

#endif
