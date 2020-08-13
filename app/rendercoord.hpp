#ifndef RENDER_THREAD_H
#define RENDER_THREAD_H

#include "draw.hpp"
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
        RenderThread(Renderer r, QObject *parent = nullptr, unsigned id = 0);

        // Returns 0 if successful or 1 if busy
        int render(QRgb *buffer, unsigned samples);
        unsigned current_samples();

    signals:
        void done(unsigned workerid);

    protected:
        void run();

        QSemaphore m_lock;

        QRgb *m_writebuffer;
        unsigned m_samples;
        unsigned m_current_samples;

        Renderer m_render;

        // Value in here means work is to be done
        QSemaphore m_work;
        unsigned m_id;
};

const std::string states[] = { "Stopped", "Running" };
enum State { stopped, running };

class RenderCoordinator : public QObject {
    Q_OBJECT

    public:
        RenderCoordinator(QObject *parent, DrawWidget &target, Renderer r, QLabel *status=nullptr);
        void setSamples(unsigned samples);
        void render();

    public slots:
        void workerDone(unsigned workerid);

    private slots:
        void updateUi();

    private:
        DrawWidget &m_target;

        Renderer m_renderer;
        RenderThread m_worker;

        QLabel *m_status;
        QTimer m_timer;

        State m_state;

        unsigned m_samples;
};

#endif
