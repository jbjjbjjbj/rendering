#include "rendercoord.hpp"
#include <algorithm>
#include <qobject.h>
#include <iostream>
#include <qrgb.h>

#include <qsemaphore.h>
#include <render.hpp>
#include <sstream>

uint32_t colorToUint32(const Spectrum &c) {
    Spectrum cnew = c.clamp(0, 1);
    cnew *= 255;
    return (0xFF << 24) + 
        ((int)cnew.R() << 16) +
        ((int)cnew.G() << 8) +
        cnew.B();
}

// Run by main thread
RenderThread::RenderThread(Renderer r, unsigned threads, const Config &conf, QObject *parent, unsigned id) 
    : QThread(parent),
    m_lock(1),
    m_render(r),
    m_conf(conf),
    m_pause(1)
{
    m_id = id;
    m_workers = threads;
}

// Run on new thread
void RenderThread::run() {
    while (1) {
        // Wait for work
        m_work.acquire();

        // Very expensive, but necesary to get live rendering
        Spectrum *sum = new Spectrum[m_render.m_width * m_render.m_height];

        m_current_samples = 0;

        for (int sample = 0; sample < m_samples; sample++) {
            m_current_samples = sample;
            // Probably not that smart
            m_pause.acquire();
            m_pause.release();

            for (unsigned y = m_id; y < m_render.m_height; y += m_workers) {
                for (unsigned x = 0; x < m_render.m_width; x++) {
                    auto index = x + y * m_render.m_width;
                    sum[index] += m_render.render(m_render.m_width - x, m_render.m_height - y, 1);

                    m_writebuffer[index] = colorToUint32(sum[index] / (sample+1));
                }
            }

        }

        // Signal done
        m_lock.release();
        emit done(m_id);
    }
}

void RenderThread::pause() {
    m_pause.acquire();
}

void RenderThread::resume() {
    m_pause.release();
}

int RenderThread::render(QRgb *buffer, unsigned samples) {
    // Check if already running
    if (!m_lock.tryAcquire()) {
        return 1;
    }
    m_writebuffer = buffer;
    m_samples = samples;
    m_work.release();
    std::cout << samples << std::endl;

    return 0;
}

unsigned RenderThread::stop() {
    stopAt(m_current_samples);
    return m_current_samples;
}

void RenderThread::stopAt(int at) {
    m_samples = at;
}

// Running on main thread
unsigned RenderThread::current_samples() {
    // No sync should not be a problem here.
    return m_current_samples;
}

RenderCoordinator::RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r, const Config &conf, QLabel *status) 
    : QObject(parent),
    m_target(target),
    m_renderer(r),
    m_timer(this),
    m_conf(conf)
{
    m_status = status;

    // Create and start workers
    for (unsigned i = 0; i < conf.m_workers; i++) {
        auto thread = new RenderThread(m_renderer, conf.m_workers, conf, this, i);

        thread->start();
        QObject::connect(thread, &RenderThread::done, this, &RenderCoordinator::workerDone);

        m_workers.push_back(thread);
    }

    render();

}

void RenderCoordinator::render() {
    m_started = 0;
    for (auto thd : m_workers) {
        thd->render(m_target.m_drawbuffer, m_conf.m_samples);
        m_started++;
    }

    m_state = running;
    updateUi();

    QObject::connect(&m_timer, &QTimer::timeout, this, &RenderCoordinator::updateUi);

    m_timer.start(1000.0 / m_conf.m_framerate);
}

void RenderCoordinator::stop() {
    unsigned max = 0;
    for (auto thd : m_workers) {
        thd->pause();

        auto val = thd->current_samples();
        if (val>max) {
            max = val;
        }
    }

    std::cout << max << std::endl;

    for (auto thd : m_workers) {
        thd->stopAt(max+1);
        thd->resume();
    }

    m_state = stopping;
    updateUi();
}

void RenderCoordinator::workerDone(unsigned workerid) {
    std::cout << "Worker " << workerid << " done!" << std::endl;
    if (--m_started) {
        return;
    }
    std::cout << "All done :-)" << std::endl;

    // All workers are done
    m_state = stopped;
    m_timer.stop();
    updateUi();
}

unsigned RenderCoordinator::calcStats(unsigned *max, unsigned *min, double *avg) {
    unsigned count = 0;
    unsigned sum = 0;
    for (auto thd : m_workers) {
        auto val = thd->current_samples();
        if (min && (val < *min || !count)) {
            *min = val;
        }
        if (max && (val > *max || !count)) {
            *max = val;
        }

        sum += val;
        count++;
    }

    if (avg) {
        *avg = (double)sum / count;
    }

    return count;
}


void RenderCoordinator::updateUi() {
    m_target.repaint();

    if (!m_status) {
        return;
    }

    // Gather statictics from workers
    unsigned max;
    unsigned min;
    double avg;
    unsigned count = calcStats(&max, &min, &avg);

    std::ostringstream status;
    status << states[m_state] << 
        " Threads: " << count <<
        " Max: " << max << " samples" <<
        " Min: " << min << " samples" <<
        " Avg: " << avg << " samples";

    m_status->setText(QString::fromStdString(status.str()));
}
