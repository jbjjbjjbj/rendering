#include <object.hpp>
#include <ray.hpp>
#include <common.hpp>
#include <vector.hpp>

#include <catch2/catch.hpp>
#include <math.h>

TEST_CASE("Sphere normal at", "[sphere]") {
    auto sph = Sphere(Vec3d(2, 3, 4), 2);

    auto norm = sph.norm_at(Vec3d(2, 3, 2), Vec3d());
    REQUIRE(norm.m_x == 0);
    REQUIRE(norm.m_y == 0);
    REQUIRE(norm.m_z == -1);
}

TEST_CASE("Sphere intersect", "[sphere]") {
    auto sph = Sphere(Vec3d(2, 3, 4), 2);
    auto ray = Ray(Vec3d(1, 0, 0), Vec3d(0, 1, 1.5), true);

    auto dist = sph.intersect(ray, false);
    REQUIRE(abs(dist - 3.28) < 0.01);
}

TEST_CASE("Plane intersect", "[plane]") {
    auto pln = Plane(Vec3d(3, 4, 2), Vec3d(-6, -3, -2));
    auto ray = Ray(Vec3d(0, 0, 0), Vec3d(-2, -1, 5));

    auto dist = pln.intersect(ray, false);
    REQUIRE(dist == -1);

    ray = Ray(Vec3d(-2, -2, 0), Vec3d(-2, -1, 5));
    dist = pln.intersect(ray, false);
    REQUIRE(abs(dist - 20.4) < 0.1);
}
