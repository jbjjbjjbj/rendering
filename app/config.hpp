#ifndef CONFIG_H
#define CONFIG_H

#include <render.hpp>

class Config {
    public:
        unsigned m_width, m_height;
        unsigned m_maxhops, m_samples;
        
        unsigned m_framerate, m_workers;
};

class RendererConf : public Renderer {
    public:
        RendererConf(const Scene &scn, Vec3d eye, Vec3d target, Config &conf) 
            : Renderer(scn, eye, target, conf.m_width, conf.m_height, conf.m_maxhops) {
            
            }
};

#endif
