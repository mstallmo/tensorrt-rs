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

fn main() {
    let uff_file = UffFile::new(Path::new("../assets/sample_ssd_relu6.uff")).unwrap();
    let engine = create_engine(&uff_file);
    println!("Nb outputs: {}", engine.get_nb_bindings());

    let context = engine.create_execution_context();
    let input_image = image::open("../assets/images/dog.ppm").unwrap().into_bgr();

    let array: ndarray_image::NdColor = ndarray_image::NdImage(&input_image).into();
    println!("NdArray shape: {:?}", array.shape());
    let transposed = array.t();
    println!("NdArray shape transposed: {:?}", transposed.shape());
    let pre_processed = Array::from_iter(transposed.iter().map(|&x| 1.0 - (x as f32) / 255.0));

    let binding_dim = engine.get_binding_dimensions(1);
    let dim_slice= &binding_dim.d()[0..binding_dim.nb_dims() as usize];
    let vol = dim_slice.iter().fold(1, |acc, x| acc * x);
    let mut output = ndarray::Array1::<f32>::zeros(vol as usize);

    //TODO: Execution fails because of an incorrect number of output bindings. SSD has 2 outputs and
    // we've only bound 1. This causes the nmsPlugin to fail the assertion in enqueue. Line 118.
    // Add functionality to bind more output for an engine to fix this issue.
    context.execute(&pre_processed, 0, &mut output, 1);
    println!("output: {}", output);

    println!("Done!");
}