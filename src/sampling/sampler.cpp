#include "sampler.hpp"

void Sampler::startPixel(const Vec2d &p) {
    m_curPixel = p;
    m_curPixelSampleIndex = 0;
}

bool Sampler::startNextSample() {
    return ++m_curPixelSampleIndex < m_sampleCount;
}
