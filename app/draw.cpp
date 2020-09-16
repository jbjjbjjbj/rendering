#include "draw.hpp"
#include <qnamespace.h>
#include <qpainter.h>
#include <qglobal.h>
#include <qimage.h>
#include <qrgb.h>
#include <qtimer.h>
#include <qwindowdefs.h>
#include <iostream>

DrawWidget::DrawWidget(const Config &conf) : 
    QWidget(), m_conf(conf) {
    m_drawbuffer = new QRgb[conf.m_width * conf.m_height];

    m_img = QImage((uchar*)m_drawbuffer, conf.m_width, conf.m_height, QImage::Format_ARGB32);

}

void DrawWidget::paintEvent(QPaintEvent*) {
    QPainter painter(this);

    auto scaled = m_img.scaled(width(), height(), Qt::KeepAspectRatio);
    painter.drawImage(0, 0, scaled);
}

void DrawWidget::redraw() {
    repaint();
}

DrawWidget::~DrawWidget() {
    delete[] m_drawbuffer;
}
