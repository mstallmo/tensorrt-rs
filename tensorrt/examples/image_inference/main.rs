use tensorrt_rs::runtime::Logger;
use tensorrt_rs::builder::Builder;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::context;
use std::path::Path;
use std::mem;
use tensorrt_rs::uff::{UffInputOrder, UffParser, UffFile};
use tensorrt_rs::dims::DimsCHW;
use image::GenericImageView;
use std::panic::resume_unwind;


fn create_engine(uff_file: &UffFile) -> Engine {
    let logger = Logger::new();
    let builder = Builder::new(&logger);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(1, 28, 28);
    uff_parser.register_input("in", dim, UffInputOrder::Nchw).unwrap();
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
    let input_image = image::open("../assets/images/0.pgm").unwrap();
    println!("Image dimensions: {:?}", input_image.dimensions());
    println!("Image color: {:?}", input_image.color());

    // Pre-process the data read from the .pgm image to match model input
    let image_bytes: Vec<f32> = input_image.to_bytes().iter().map(|&x| 1.0 - (x as f32) / 255.0).collect();
    println!("Image bytes size: {}", image_bytes.len());


    // Need to pre-allocate the Vector for this to work. Just calling with capacity doesn't seem to
    // allocate the underlying data.
    let mut result = vec![0.0; 10];
    context.execute(&image_bytes,
                    image_bytes.len() * mem::size_of::<f32>(),
                    0, &mut result,
                    10 * mem::size_of::<f32>(), 1);

    let (idx, maxVal) = result.iter().enumerate().fold((0, 0.0f32), |(mut idx, mut max), (valIdx, &val)| {
        if val > max {
            max = val;
            idx = valIdx
        }
        (idx, max)
    });

    println!("Image is a {} with a value of {}", idx, maxVal);
}