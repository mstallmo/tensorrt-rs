use ndarray::Array;
use ndarray_image;
use std::iter::FromIterator;
use std::path::Path;
use tensorrt_rs::builder::Builder;
use tensorrt_rs::dims::DimsCHW;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::runtime::Logger;
use tensorrt_rs::uff::{UffFile, UffInputOrder, UffParser};

fn create_engine(uff_file: &UffFile) -> Engine {
    let logger = Logger::new();
    let builder = Builder::new(&logger);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(1, 28, 28);
    uff_parser
        .register_input("in", dim, UffInputOrder::Nchw)
        .unwrap();
    uff_parser.register_output("out").unwrap();
    uff_parser.parse(uff_file, builder.get_network()).unwrap();

    builder.build_cuda_engine()
}

fn main() {
    // Create TensorRT engine from .uff file
    let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
    let engine = create_engine(&uff_file);

    // Create execution context
    let context = engine.create_execution_context();

    // Load image from disk
    let input_image = image::open("../assets/images/0.pgm").unwrap().into_luma();
    println!("Image dimensions: {:?}", input_image.dimensions());

    // Convert image to ndarray
    let array: ndarray_image::NdGray = ndarray_image::NdImage(&input_image).into();
    println!("NdArray len: {}", array.len());
    let pre_processed = Array::from_iter(array.iter().map(|&x| 1.0 - (x as f32) / 255.0));

    // Run inference
    let mut output = ndarray::Array1::<f32>::zeros(10);
    let outputs = vec![&mut output];
    context.execute(&pre_processed, &outputs, 2);
    println!("output: {}", outputs[0]);
}
