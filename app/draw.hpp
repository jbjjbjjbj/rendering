#ifndef DRAW_H
#define DRAW_H

#include <qimage.h>
#include <qwidget.h>

class DrawWidget : public QWidget {
    public:
        DrawWidget(unsigned width, unsigned height);
        void paintEvent(QPaintEvent*);

        QRgb *m_drawbuffer;
        unsigned m_width, m_height;

        ~DrawWidget();
    private:
        QImage m_img;
        unsigned char i;
};

#endif
