#include "rendercoord.hpp"
#include <qobject.h>
#include <iostream>
#include <qrgb.h>

#include <render.hpp>
#include <sstream>

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

        // Very expensive, but necesary to get live rendering
        Color *sum = new Color[m_render.m_width * m_render.m_height];

        m_current_samples = 0;

        for (unsigned sample = 1; sample < m_samples+1; sample++) {
            for (unsigned x = 0; x < m_render.m_width; x++) {
                for (unsigned y = 0; y < m_render.m_height; y++) {
                    auto index = x + y * m_render.m_height;
                    sum[index] += m_render.render(m_render.m_width - x, m_render.m_height - y, 1);

                    m_writebuffer[index] = colorToUint32(sum[index] / sample);
                }
            }

            m_current_samples = sample;
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

// Running on main thread
unsigned RenderThread::current_samples() {
    // No sync should not be a problem here.
    return m_current_samples;
}

RenderCoordinator::RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r, QLabel *status) 
    : QObject(parent),
    m_target(target),
    m_renderer(r),
    m_worker(m_renderer, this),
    m_timer(this)
{
    m_status = status;

    m_worker.start();

    QObject::connect(&m_worker, &RenderThread::done,
            this, &RenderCoordinator::workerDone);

    m_worker.render(target.m_drawbuffer, 100);

    m_state = running;
    updateUi();

    QObject::connect(&m_timer, &QTimer::timeout, this, &RenderCoordinator::updateUi);

    m_timer.start(500);

}

void RenderCoordinator::workerDone(unsigned workerid) {
    m_state = stopped;
    m_timer.stop();
    updateUi();
}


void RenderCoordinator::updateUi() {
    m_target.repaint();

    if (!m_status) {
        return;
    }

    std::ostringstream status;
    status << states[m_state] << " " << m_worker.current_samples() << " samples";

    m_status->setText(QString::fromStdString(status.str()));
}
