use std::iter::FromIterator;
use std::path::PathBuf;

use ndarray::Array;
use ndarray_image;
use tensorrt_rs::builder::{Builder, NetworkBuildFlags};
use tensorrt_rs::dims::Dims4;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::onnx::{OnnxFile, OnnxParser};
use tensorrt_rs::runtime::Logger;

///
/// Some strange bug with lifetimes of different trt objects. They should all be alive.
///
pub struct TensorRT {
    logger: Logger,
    engine: Engine,
    builder: Builder,
}

impl TensorRT {
    pub fn new(file: OnnxFile, batch_size: i32, workspace_size: usize) -> Self {
        let logger = Logger::new();
        let builder =
            Builder::new_v2(&logger, NetworkBuildFlags::EXPLICIT_BATCH);
        let engine = {
            let network = builder.get_network();
            let parser = OnnxParser::new(network, &logger);
            let verbosity = 7;

            builder.set_max_batch_size(batch_size);
            builder.set_max_workspace_size(workspace_size);

            parser.parse_from_file(&file, verbosity).unwrap();

            let dim = Dims4::new(batch_size, 224, 224, 3);
            network.get_input(0).set_dimensions(dim);
            builder.build_cuda_engine()
        };

        Self {
            logger,
            engine,
            builder,
        }
    }
}

fn main() {
    let file = OnnxFile::new(&PathBuf::from("../assets/efficientnet.onnx")).unwrap();
    let gb = 1024 * 1024 * 1024;
    let trt = TensorRT::new(file, 1, 1 * gb);

    let context = trt.engine.create_execution_context();

    let input_image = image::open("../assets/images/meme.jpg")
        .unwrap()
        .crop(0, 0, 100, 100)
        .into_rgb();
    eprintln!("Image dimensions: {:?}", input_image.dimensions());

    // Convert image to ndarray
    let array: ndarray_image::NdColor =
        ndarray_image::NdImage(&input_image).into();
    println!("NdArray len: {}", array.len());

    let pre_processed =
        Array::from_iter(array.iter().map(|&x| 1.0 - (x as f32) / 255.0));

    // Run inference
    let mut output = ndarray::Array1::<f32>::zeros(1000);
    context.execute(&pre_processed, 0, &mut output, 1);
    println!("output: {}", output);
}
