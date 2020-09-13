# TensorRT-RS
![Crates.io](https://img.shields.io/crates/v/tensorrt-rs)

:warning: __This crate currently only supports Linux__ :warning:

Rust library for creating and executing TensorRT engines.

This library depends on tensorrt-sys for Rust bindings to the underlying C++ TensorRT library. See the tensorrt-sys
README for information on prerequisite dependencies.

### Status
This crate is still very much in early stage devleopment. Support for TensorRT functionality is only the basic of what 
is needed to read a model file in .uff format, parse that file into a TensorRT engine and execute that engine, and seralize
 a binary version of that engine to disk. Currently TensorRT plugins are not supported so there may be issues when trying 
 to use a model that requires plugin support to operate.


### Upcoming Improvements to the Project
- Feature configuration for selecting the TensorRT library version
- Support for CUDA streams for async execution
- Support for the TensorRT Onnx Parser
- Support for all functionality in the TensorRT C++ library
    - Complete support for the classes that already have bindings.
    - Add support for loading custom plugins.
    - Add support for custom layer implementations

Upcoming features are not constrained to those listed above. Any feature requests are welcome and appreciated!

### Examples
See the examples directory for a basic example on loading a UFF model, creating an engine, and performing inference on 
an image using the engine. The sample uses the UFF MNIST model provided in the samples directory when installing TensorRT
on Linux.

Contributions are always welcome! Feel free to reach out with any questions or improvements and I will try to get back
to you as soon as possible! 