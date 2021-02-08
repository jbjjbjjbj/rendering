# Rendering in rust

This is a (not finished) ray tracer written in rust.
It is based on the book serie [Ray Tracing In One Weekend](https://raytracing.github.io/) and [Physics Based Rendering](http://www.pbr-book.org/).

This rust version is my third implementation, the previous being:


- [Raytracing in c](https://git.jtle.dk/raytrace/about/)
    This is the one which gives the best results,
    however as my ambitions for the project grew as did my requirement for more language features.
- [Pathtracing in rust](https://git.jtle.dk/pathtrace/about/)
I never really finished it before changing to rust.
    However it had a really cool qt gui, updating as samples were added.

## Rendered image

![render](https://git.jtle.dk/pathtrace/plain/generated.png?h=rendered)

## Goals

- [X] Render collection of circles outside, with blurry background
- [ ] Render [Cornell box](https://en.wikipedia.org/wiki/Cornell_box)
- [ ] Render [Utah teapot](https://en.wikipedia.org/wiki/Utah_teapot)
- [ ] Render [Stanford dragon](https://en.wikipedia.org/wiki/Stanford_dragon)
