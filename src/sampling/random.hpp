#ifndef SAMPLER_RANDOM_H
#define SAMPLER_RANDOM_H

#include "sampler.hpp"
#include <core/random.hpp>

class RandomSampler : public Sampler {
    public:
        RandomSampler(long samples, int seed = 0);

        double getSample();
        Vec2d get2dSample();

    private:
        Random m_rng;
};

#endif
