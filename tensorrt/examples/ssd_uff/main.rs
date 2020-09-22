use ndarray_image;
use ndarray::Array;
use std::path::Path;
use std::iter::FromIterator;
use tensorrt_rs::runtime::Logger;
use tensorrt_rs::builder::Builder;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::uff::{UffInputOrder, UffParser, UffFile};
use tensorrt_rs::dims::DimsCHW;

fn create_engine(uff_file: &UffFile) -> Engine {
    let logger = Logger::new();
    let builder = Builder::new(&logger);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(3, 300, 300);
    uff_parser.register_input("Input", dim, UffInputOrder::Nchw).unwrap();
    uff_parser.register_output("NMS").unwrap();
    uff_parser.parse(uff_file, builder.get_network()).unwrap();

    builder.build_cuda_engine()
}

//TOOD: Add get dims function to engine that returns all the dims for each binding.
// Figure out sutiable algorithm for figuring out how to calculate the needed space based on the
// dims information provided by the engine. See https://github.com/NVIDIA/TensorRT/blob/release/5.1/samples/common/buffers.h
// for a C++ example. Could think about putting it as a value on the Engine struct itself?
fn main() {
    let uff_file = UffFile::new(Path::new("../assets/sample_ssd_relu6.uff")).unwrap();
    let engine = create_engine(&uff_file);
    println!("Nb outputs: {}", engine.get_nb_bindings());
    // println!("Binding dims: {}", engine.get)

    let context = engine.create_execution_context();
    let input_image = image::open("../assets/images/dog.ppm").unwrap().into_bgr();

    let array: ndarray_image::NdColor = ndarray_image::NdImage(&input_image).into();
    println!("NdArray shape: {:?}", array.shape());
    let transposed = array.t();
    println!("NdArray shape transposed: {:?}", transposed.shape());
    let pre_processed = Array::from_iter(transposed.iter().map(|&x| 1.0 - (x as f32) / 255.0));


    println!("Done!");
}