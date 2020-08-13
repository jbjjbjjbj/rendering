#ifndef DRAW_H
#define DRAW_H

#include <qimage.h>
#include <qtimer.h>
#include <qwidget.h>

class DrawWidget : public QWidget {
    Q_OBJECT

    public:
        DrawWidget(unsigned width, unsigned height);
        void paintEvent(QPaintEvent*);

        QRgb *m_drawbuffer;
        unsigned m_width, m_height;

        ~DrawWidget();
    private slots:
        void redraw();

    private:
        QImage m_img;
        unsigned char i;

        QTimer m_timer;
};

#endif
