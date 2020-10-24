use std::iter::FromIterator;
use std::path::PathBuf;

use ndarray::Array;
use ndarray_image;
use tensorrt_rs::builder::{Builder, NetworkBuildFlags};
use tensorrt_rs::context::ExecuteInput;
use tensorrt_rs::data_size::GB;
use tensorrt_rs::dims::Dims4;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::onnx::{OnnxFile, OnnxParser};
use tensorrt_rs::runtime::Logger;

fn create_engine(
    logger: &Logger,
    file: OnnxFile,
    batch_size: i32,
    workspace_size: usize,
) -> Engine {
    let builder = Builder::new(&logger);
    let network = builder.create_network_v2(NetworkBuildFlags::EXPLICIT_BATCH);
    let verbosity = 7;

    builder.set_max_batch_size(batch_size);
    builder.set_max_workspace_size(workspace_size);

    let parser = OnnxParser::new(&network, &logger);
    parser.parse_from_file(&file, verbosity).unwrap();

    let dim = Dims4::new(batch_size, 224, 224, 3);
    network.get_input(0).set_dimensions(dim);
    builder.build_cuda_engine(&network)
}

fn main() {
    let logger = Logger::new();
    let file = OnnxFile::new(&PathBuf::from("../assets/efficientnet.onnx")).unwrap();
    let engine = create_engine(&logger, file, 1, 1 * GB);

    let context = engine.create_execution_context();

    let input_image = image::open("../assets/images/meme.jpg")
        .unwrap()
        .crop(0, 0, 100, 100)
        .into_rgb();
    eprintln!("Image dimensions: {:?}", input_image.dimensions());

    // Convert image to ndarray
    let array: ndarray_image::NdColor = ndarray_image::NdImage(&input_image).into();
    println!("NdArray len: {}", array.len());

    let pre_processed = Array::from_iter(array.iter().map(|&x| 1.0 - (x as f32) / 255.0));

    // Run inference
    let mut output = ndarray::Array1::<f32>::zeros(1000);
    let outputs = vec![ExecuteInput::Float(&mut output)];
    context.execute(ExecuteInput::Float(&pre_processed), outputs, 2);
    println!("output: {}", output);
}
