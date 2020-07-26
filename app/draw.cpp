#include "draw.hpp"
#include <qpainter.h>
#include <qglobal.h>
#include <qimage.h>
#include <qrgb.h>
#include <qwindowdefs.h>
#include <iostream>

DrawWidget::DrawWidget(unsigned width, unsigned height) : QWidget() {
    m_width = width;
    m_height = height;
    m_drawbuffer = new QRgb[width * height];

    m_img = QImage((uchar*)m_drawbuffer, width, height, QImage::Format_ARGB32);
}

void DrawWidget::paintEvent(QPaintEvent*) {
    QPainter painter(this);
    painter.drawImage(0, 0, m_img);
}

DrawWidget::~DrawWidget() {
    delete[] m_drawbuffer;
}
