#ifndef SCENE_H
#define SCENE_H

#include <vector>
#include "object.hpp"

class Scene {
    public:
        void addObject(Object obj);

        std::vector<Object> objs;

};

#endif
