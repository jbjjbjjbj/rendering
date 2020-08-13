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
        QImage m_img;
        unsigned m_width, m_height;

        ~DrawWidget();
    private slots:
        void redraw();

    private:
        unsigned char i;
};

#endif
