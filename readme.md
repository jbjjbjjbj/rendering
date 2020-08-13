# Pathtracing

Second attempt at a renderer, written in C++ instead of C to compare languages.
This is also my first big project in C++.

My first attempt can be seen here [github](https://github.com/jbjjbjjbj/raytrace) and here [cgit](https://git.jtle.dk/raytrace/about/).

## Preview

![render](https://git.jtle.dk/pathtrace/plain/generated.png?h=rendered)

## Build requirements

- Catch2
- Qt5

## Building

This is just standard cmake building.

```
mkdir build
cd build
cmake ../
make
```

The program does currently not have any options, so just run it.

```
./pathtracing
```


## Testing

To test the program follow the steps above but instead of `make` run `make test`.

```
make test
```

Then run the test executable.
Remember to rerun `make test` when changing source files.

```
./run_test
```
