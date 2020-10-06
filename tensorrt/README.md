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
- ~~Feature configuration for selecting the TensorRT library version~~ - added in 0.4.0
- Support for CUDA streams for async execution
- ~~Support for the TensorRT Onnx Parser~~ - added in 0.4.0
- Support for all functionality in the TensorRT C++ library
    - Complete support for the classes that already have bindings.
    - Add support for loading custom plugins.
    - Add support for custom layer implementations

Upcoming features are not constrained to those listed above. Any feature requests are welcome and appreciated!

### Usage

#### TensorRT version
TensorRT-RS supports multiple versions of bindings to the underlying TensorRT library. The current supported versions 
are TensorRT 5, TensorRT 6, and TensorRT 7. The various bindings are enabled via cargo features. By default bindings 
are generated for TensorRT 5. If you need to bind to one of the other supported versions of TensorRT set the feature 
in your `Cargo.toml`.
```
[dependencies]
tensorrt-rs = { version = 0.4.0, default-features = false, features = ["trt-7"] }
``` 

#### TensorRT Library in Non default Location
By default TensorRT will link to the TensorRT dylibs that are installed in the system library directory on Linux 
(there isn't a Windows default install location). If you would like to use a different version of TensorRT that is not 
in the default location you can set the path to the libraries via the `TRT_INSTALL_DIR` environment variable. The env 
variable should point to the root of the TensorRT folder. The build process will append `/lib` and `/include` to the `
TRT_INSTALL_DIR` where appropriate.

ex.
```shell script
export TRT_INSTALL_DIR=~/TensorRT-7.1.3.4
```

#### CUDA Library in Non-default Location
Related to linking to a [library in non-default location](#TensorRT-Library-in-Non-default-Location) we also support 
linking to a non default CUDA install location. When using a different TensorRT library than is installed in the default 
location on your system it's likely that the CUDA version that is installed will not be correct for that TensorRT 
version. This is also done via an environment variable `CUDA_INSTALL_DIR`. This variable should be set to the root of 
the directory as well such that appending `lib64` will result in finding the appropriate CUDA libraries. 

ex.
```shell script
export CUDA_INSTALL_DIR=~/cuda-10.1
```

### Examples
See the examples directory for a basic example on loading a UFF model, creating an engine, and performing inference on 
an image using the engine. The sample uses the UFF MNIST model provided in the samples directory when installing TensorRT
on Linux.

Contributions are always welcome! Feel free to reach out with any questions or improvements and I will try to get back
to you as soon as possible! 