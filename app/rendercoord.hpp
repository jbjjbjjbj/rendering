#ifndef RENDER_THREAD_H
#define RENDER_THREAD_H

#include "draw.hpp"
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
        RenderThread(Renderer r, QObject *parent = nullptr, unsigned id = 0);

        // Returns 0 if successful or 1 if busy
        int render(QRgb *buffer, unsigned samples);

    signals:
        void done(unsigned workerid);

    protected:
        void run();

        QSemaphore m_lock;

        QRgb *m_writebuffer;
        unsigned m_samples;

        Renderer m_render;

        // Value in here means work is to be done
        QSemaphore m_work;
        unsigned m_id;
};

class RenderCoordinator : public QObject {
    Q_OBJECT

    public:
        RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r);
        void setSamples(unsigned samples);
        void render();

    public slots:
        void workerDone(unsigned workerid);

    private:
        DrawWidget &m_target;

        Renderer m_renderer;
        RenderThread m_worker;

        unsigned m_samples;
};

#endif
