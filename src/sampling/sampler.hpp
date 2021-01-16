#ifndef SAMPLER_H
#define SAMPLER_H

#include <core/vector.hpp>

#include <memory>

static const double LargestBelowOne = 0x1.fffffffffffffp-1;

class Sampler {
    public:
        Sampler(long sampleCount) : m_sampleCount(sampleCount) {}

        /** @brief Start a new pixel sample
         *
         * Some samplers might find it usefull to get the position 
         */
        void startPixel(const Vec2d &p);

        /** @brief Called by renderers when a new sample begins 
         *
         * @return Whether this is the last sample
         */
        bool startNextSample();

        /** @brief Get a single sample */
        virtual double getSample() = 0;

        /** @brief Get a 2d sample
         *
         * Algorithms can optimize for getting 2d samples
         */
        virtual Vec2d get2dSample() = 0;

        /** @brief Clones the sampler
         *
         * Usefull when multiple threads need a sampler.
         * 
         * @param seed Seed to set in cloned sampler
         */
        virtual std::unique_ptr<Sampler> clone(int seed) = 0;

        const long m_sampleCount;

    protected:
        Vec2d m_curPixel;
        long m_curPixelSampleIndex;

};


#endif
