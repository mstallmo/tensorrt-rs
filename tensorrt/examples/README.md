# Examples

This directory contains example applications for using the TensorRT-rs library. These are small examples that demonstrate
basic usage of the library and associated data types to run model inference using TensorRT. Examples are named for the 
model used as well as the file format output from the training framework (uff or onnx). All sample data and models ship 
with the TensorRT library install from nvidia.

These examples are adapted from the C++ examples that are provided with TensorRT. The logic is not a 1 for 1 port but the
basic model inference idea is the same.

More examples are always welcome. Please feel free to open a PR with a new example if you feel that one is missing from
the list here.

## Getting Started

All examples expect models and input data to be available in the `assets` subfolder that resides in the top level
directory. This folder is not tracked by git and will need to be crated and populated on your machine.

Models and input data are shipped with TensorRT and should be copied into the assets folder. Models live at the top level
and images live in the images sub-folder. 

If you installed TensorRT via the .deb archive on Linux these assets can be found in `/usr/src/tensorrt/data`. In this 
folder each of the assets will be split by specific example sub-folder (mnist, ssd, etc). 

## mnist_uff
This example uses the MNIST digit dataset and classifies images of hand drawn digits. The output is 10 entries representing
the numbers 0..9 and will have an associated floating point value that indicates what digit the model identified. 

For more information about this example see: 
[TensorRT MNIST Sample](https://github.com/mstallmo/TensorRT/tree/master/samples/opensource/sampleUffMNIST)

### Run
```shell script
$ cargo run --example mnist_uff
```

## ssd_uff
This example uses the Single Shot MultiBox Detector to perform object detection in images. The output of the model will 
be the bounding box data and confidence scores. This plugin takes advantage of the TensorRT plugins provided by nvidia,
specifically the NMS (non-max suppression) plugin.

To properly map the operations in the UFF graph to the appropriate TensorRT plugin there is a small preprocessing step 
that needs to be done to the model before it can be loaded by TensorRT in the example. The details can be found in the 
[TensorRT SSD Sample](https://github.com/mstallmo/TensorRT/tree/master/samples/opensource/sampleUffSSD#sampleuffssd-plugins)

The result of this pre-processing should be placed into the `assets` in the top level directory.

### Run
```shell script
$ cargo run --example ssd_uff
```