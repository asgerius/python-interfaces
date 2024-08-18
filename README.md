# Python interfaces

This repository contains example of how to call functions written in fast languages from Python.
The base example is element-wise multiplication of an array.

## Requirements

- `python` and `pip` because of course
- `gcc` for compiling C code
- `g++` for compiling C++ code
- `nvcc` and `cuda` for CUDA code
- `cargo` for compiling Rust code

These should be available using your system's package manager.

To install the Python package, run `make install`.
Then, build the libraries (for which the relevant tools are installed) using
```sh
make c
make cpp
make cuda  # Still WIP, so this will not work
make rust
```
All commands are made for Linux, so you may have to tinker with it if you are on Windows.
One notable change is that the files should be compiled to `.dll` instead of `.so`.
