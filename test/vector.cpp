#include <vector.hpp>
#include <common.hpp>
#include <catch2/catch.hpp>

TEST_CASE( "Vector length", "[vector]" ) {
    auto vec = Vec3d(2, 4, 4);
    REQUIRE(vec.length() == 6);
    vec.set(0, 0, 0);
    REQUIRE(vec.length() == 0);
    vec.set(0, 3.5, 0);
    REQUIRE(vec.length() == 3.5);
}

TEST_CASE("Vector_normal", "[vector]") {
    auto vec = Vec3d(4, 5, 4545);
    REQUIRE(vec.length() != 1.0);
    vec.normalize();
    REQUIRE(vec.length() - 1.0 < ZERO_APPROX);
    vec.set(0, 0, 0);
    REQUIRE_THROWS(vec.normalize());
}

TEST_CASE("Vector dot", "[vector]") {
    auto a = Vec3d(4, 5, 6);
    auto b = Vec3d(1, 2, 3);
    REQUIRE(a.dot(b) == 32);
    a.set(0, 0, 0);
    REQUIRE(a.dot(b) == 0);
    a.set(0, 5, 0);
    REQUIRE(a.dot(b) == 10);
}
