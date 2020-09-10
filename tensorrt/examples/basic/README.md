# Basic Example

This example demonstrates the basic creating of a TensorRT engine from 
a .uff file produced by any of the major deep learning frameworks. Specifically
this example uses the MNIST .uff file provided in the samples of the TensorRT
install.

In this example we create a TensorRT engine from the path to the .uff file's location
and print out some basic information about the created engine.

To run this example:
```
cargo run --example basic
``` 