#include "rendercoord.hpp"
#include <qobject.h>
#include <iostream>
#include <qrgb.h>

#include <render.hpp>

uint32_t colorToUint32(const Color &c) {
    Color cnew = Color(c);
    cnew.clamp();
    return (0xFF << 24) + 
        (cnew.r() << 16) +
        (cnew.g() << 8) +
        cnew.b();
}

// Run by main thread
RenderThread::RenderThread(Renderer r, QObject *parent, unsigned id) 
    : QThread(parent),
    m_lock(1),
    m_render(r)
{
    m_id = id;
}

// Run on new thread
void RenderThread::run() {
    while (1) {
        // Wait for work
        m_work.acquire();
        m_render.m_sampler.seed(100);

        // Very expensive, but necesary to get live rendering
        Color *sum = new Color[m_render.m_width * m_render.m_height];

        for (unsigned sample = 1; sample < m_samples+1; sample++) {
            for (unsigned x = 0; x < m_render.m_width; x++) {
                for (unsigned y = 0; y < m_render.m_height; y++) {
                    auto index = x + y * m_render.m_height;
                    sum[index] += m_render.render(m_render.m_width - x, m_render.m_height - y, 1);

                    m_writebuffer[index] = colorToUint32(sum[index] / sample);
                }
            }
        }

        // Signal done
        m_lock.release();
        emit done(m_id);
    }
}

int RenderThread::render(QRgb *buffer, unsigned samples) {
    // Check if already running
    if (!m_lock.tryAcquire()) {
        return 1;
    }
    m_writebuffer = buffer;
    m_samples = samples;
    m_work.release();
    return 0;
}

RenderCoordinator::RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r) 
    : QObject(parent),
    m_target(target),
    m_renderer(r),
    m_worker(m_renderer, this)
{
    m_worker.start();

    QObject::connect(&m_worker, &RenderThread::done,
            this, &RenderCoordinator::workerDone);

    m_worker.render(target.m_drawbuffer, 100);

}

void RenderCoordinator::workerDone(unsigned workerid) {
    std::cout << workerid << " done!" << std::endl;
    m_target.repaint();
}

