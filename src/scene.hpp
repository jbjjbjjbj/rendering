#ifndef SCENE_H
#define SCENE_H

#include <vector>
#include "object.hpp"

class Scene {
    public:
        void addShape(const Shape *obj);

        std::vector<const Shape*> objs;

};

#endif
