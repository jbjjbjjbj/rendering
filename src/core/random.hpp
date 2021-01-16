#ifndef RANDOM_H
#define RANDOM_H

#include <random>

// TODO think more about
class Random {
    public:
        Random(int seed = 0) : m_gen(seed), m_dist(0, 1) {}

        inline double getDouble() {
            return m_dist(m_gen);
        };

    private:
        std::minstd_rand m_gen;
        std::uniform_real_distribution<double> m_dist;
};

#endif
