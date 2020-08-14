#ifndef RENDER_THREAD_H
#define RENDER_THREAD_H

#include "draw.hpp"
#include <atomic>
#include <qlabel.h>
#include <render.hpp>

#include <qobject.h>
#include <qrgb.h>
#include <qthread.h>
#include <qsemaphore.h>
#include <vector>
// https://doc.qt.io/archives/qt-4.8/qt-threads-mandelbrot-example.html

class RenderThread : public QThread {
    Q_OBJECT

    public:
        RenderThread(Renderer r, unsigned threads, QObject *parent = nullptr, unsigned id = 0);

        // Returns 0 if successful or 1 if busy
        int render(QRgb *buffer, unsigned samples);

        void pause();
        void resume();

        unsigned stop();
        void stopAt(int at);

        unsigned current_samples();

    signals:
        void done(unsigned workerid);

    protected:
        void run();

        QSemaphore m_lock;

        QRgb *m_writebuffer;
        std::atomic_int m_samples;
        std::atomic_int m_current_samples;

        Renderer m_render;

        unsigned m_workers;

        // Value in here means work is to be done
        QSemaphore m_work;
        QSemaphore m_pause;
        unsigned m_id;
};

const std::string states[] = { "Stopped", "Running", "Stopping" };
enum State { stopped, running, stopping };

class RenderCoordinator : public QObject {
    Q_OBJECT

    public:
        RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r, QLabel *status=nullptr);
        void setSamples(unsigned samples);
        void render();

    public slots:
        void workerDone(unsigned workerid);
        void stop();

    private slots:
        void updateUi();

    private:
        unsigned calcStats(unsigned *max, unsigned *min, double *avg);

        DrawWidget &m_target;

        Renderer m_renderer;
        std::vector<RenderThread*> m_workers;
        unsigned m_started;

        QLabel *m_status;
        QTimer m_timer;

        State m_state;

        unsigned m_samples;
};

#endif
