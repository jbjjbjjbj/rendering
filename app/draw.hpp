#ifndef DRAW_H
#define DRAW_H

#include "config.hpp"
#include <qimage.h>
#include <qtimer.h>
#include <qwidget.h>

class DrawWidget : public QWidget {
    Q_OBJECT

    public:
        DrawWidget(const Config &conf);
        void paintEvent(QPaintEvent*);

        QRgb *m_drawbuffer;
        QImage m_img;
        unsigned m_width, m_height;

        ~DrawWidget();
    private slots:
        void redraw();

    private:
        unsigned char i;

        const Config &m_conf;
};

#endif
