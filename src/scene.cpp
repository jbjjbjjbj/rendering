#include "scene.hpp"

void Scene::addObject(Object &obj) {
    objs.push_back(&obj);
}
