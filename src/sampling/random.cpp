#include "random.hpp"

RandomSampler::RandomSampler(long samples, int seed)
    : Sampler(samples), m_rng(seed) {}

double RandomSampler::getSample() {
    return m_rng.getDouble();
}

Vec2d RandomSampler::get2dSample() {
    return Vec2d(m_rng.getDouble(), m_rng.getDouble());
}

