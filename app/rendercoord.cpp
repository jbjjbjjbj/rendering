#include "rendercoord.hpp"
#include <qobject.h>
#include <iostream>
#include <qrgb.h>

#include <render.hpp>

uint32_t colorToUint32(Color &c) {
    c.clamp();
    return (0xFF << 24) + 
        (c.r() << 16) +
        (c.g() << 8) +
        c.b();
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

        for (unsigned x = 0; x < m_render.m_width; x++) {
            for (unsigned y = 0; y < m_render.m_height; y++) {
                auto c = m_render.render(x, y);
                m_writebuffer[x + y * m_render.m_height] = 
                    static_cast<QRgb>(colorToUint32(c));
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

    m_worker.render(target.m_drawbuffer, 1);

}

void RenderCoordinator::workerDone(unsigned workerid) {
    std::cout << workerid << " done!" << std::endl;
    m_target.repaint();
}

