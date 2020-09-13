# TensorRT-sys
![Crates.io](https://img.shields.io/crates/v/tensorrt-sys)

:warning: __This crate currently only supports Linux__ :warning:

C++ wrapper and Rust bindings to the TensorRT C++ library. Check 
[here](https://docs.nvidia.com/deeplearning/tensorrt/archives/tensorrt-515/tensorrt-api/c_api/classnvinfer1_1_1_i_builder.html) 
documentation on the C++ TensorRT library 

### Prerequisites
CUDA 10.1

TensorRT 5.1.5

CMake > 3.10



TensorRT-sys' bindings depends on TensorRT 5.1.5 for the bindings to work correctly. While other versions of
TensorRT *may* work with the bindings there are no guarantees as functions that are boudn to may have been depricated, 
removed, or changed in future verions of TensorRT.

The prerequisites enumerated above are expected to be installed in their default location on Linux 
(/usr/lib/x86_64-linux-gnu/)

### Support Matrix for TensorRT Classes
Anything not listed below currently does not have any support.

| Class Name| Status|
|------------------| ---|
| nvinfer1::ILogger| Complete|
|nvinfer1::IBuilder| Partial |
|nvinfer1::IExecutionContext| Partial|
|nvinfer1::IRuntime| Partial|
|nvinfer1::ICudaEngine| Partial|
|nvinfer1::INetworkDefinition| Partial|
|nvinfer1::IHostMemory| Partial|
|nvinfer1::IDims (and all sub-dims)| Complete|
|nvuffparser::IUffParser| Partial|




### Structure
All of the C++ code that is used to communicate between Rust and TensorRT itself is contained in the `trt-sys` sub-folder
This code exposes a C interface that can be consumed by Rust and translates between said interface and the API exposed by
the TensorRT C++ library. 

Bindings to the C++ wrapper library are generated using the bindgen command 
`bindgen --size_t-is-usize trt-sys/tensorrt_api.h -o src/bindings.rs`. All headers that make up the C api are included in
the file `tensorrt_api.h` that bindgen then consumes to crate the Rust bindings. These bindings are saved in the `src/`
folder and imported by `lib.rs` to create the crate that is used by tensorrt-rs.
