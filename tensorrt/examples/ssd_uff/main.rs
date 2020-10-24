use image::RgbImage;
use imageproc::rect::Rect;
use ndarray::{Array, Array1, Array3};
use std::iter::FromIterator;
use std::path::Path;
use tensorrt_rs::builder::{Builder, NetworkBuildFlags};
use tensorrt_rs::context::ExecuteInput;
use tensorrt_rs::dims::{Dim, DimsCHW};
use tensorrt_rs::engine::Engine;
use tensorrt_rs::runtime::Logger;
use tensorrt_rs::uff::{UffFile, UffInputOrder, UffParser};
use tensorrt_sys::Network;

fn create_engine(logger: &Logger, uff_file: &UffFile) -> Engine {
    let builder = Builder::new(&logger);
    let network = builder.create_network_v2(NetworkBuildFlags::DEFAULT);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(3, 300, 300);
    uff_parser
        .register_input("Input", dim, UffInputOrder::Nchw)
        .unwrap();
    uff_parser.register_output("NMS").unwrap();
    uff_parser.parse(uff_file, &network).unwrap();

    builder.build_cuda_engine(&network)
}

//Input formatting logic comes directly from the C++ code in the sampleUffSSD.cpp.
//https://github.com/NVIDIA/TensorRT/blob/release/5.1/samples/opensource/sampleUffSSD/sampleUffSSD.cpp
fn process_input(image: &RgbImage) -> Array1<f32> {
    let mut base_array = Array3::<f32>::zeros((3, image.height() as usize, image.width() as usize));
    for c in 0..3 {
        for j in 0..(image.height() * image.width()) as usize {
            base_array.as_slice_mut().unwrap()[c * (300 * 300) + j] =
                (2.0 / 255.0) * (image.as_flat_samples().as_slice()[j * 3 + c] as f32) - 1.0;
        }
    }
    Array::from_iter(base_array.iter().cloned())
}

fn infer(engine: &Engine, input: &Array1<f32>) -> (ndarray::Array1<f32>, ndarray::Array1<i32>) {
    let context = engine.create_execution_context();

    let binding_dim = engine.get_binding_dimensions(1);
    let dim_slice = &binding_dim.d()[0..binding_dim.nb_dims() as usize];
    let vol = dim_slice.iter().fold(1, |acc, x| acc * x) as usize;
    let mut top_detections = Array1::<f32>::zeros(vol);

    let binding_dim = engine.get_binding_dimensions(2);
    let dim_slice = &binding_dim.d()[0..binding_dim.nb_dims() as usize];
    let vol = dim_slice.iter().fold(1, |acc, x| acc * x) as usize;
    let mut keep_count = Array1::<i32>::zeros(vol);

    let outputs = vec![
        ExecuteInput::Float(&mut top_detections),
        ExecuteInput::Integer(&mut keep_count),
    ];
    let execute_input = ExecuteInput::Float(&input);
    context.execute(execute_input, outputs, 3);

    (top_detections, keep_count)
}

fn verify_output(image: &mut RgbImage, top_detections: &Array1<f32>, keep_count: &Array1<i32>) {
    for i in 0..keep_count[0] as usize {
        let det_base_index = i * 7;

        if top_detections[det_base_index + 2] > 0.5 {
            let min_x = top_detections[det_base_index + 3] * image.width() as f32;
            let min_y = top_detections[det_base_index + 4] * image.height() as f32;
            let max_x = top_detections[det_base_index + 5] * image.width() as f32;
            let max_y = top_detections[det_base_index + 6] * image.height() as f32;

            let rect = Rect::at((min_x) as i32, (min_y) as i32).of_size(
                (max_x) as u32 - (min_x) as u32,
                (max_y) as u32 - (min_y) as u32,
            );

            imageproc::drawing::draw_hollow_rect_mut(image, rect, image::Rgb([255u8, 0u8, 0u8]));

            let confidence_string = format!("confidence {}", top_detections[2] * 100.0);
            let coordinates_string =
                format!("coordinates ({}, {}), ({}, {})", min_x, min_y, max_x, max_y);
            println!(
                "Detected dog in the image with {} and {}",
                confidence_string, coordinates_string
            );
        }
    }

    image.save("test.jpg").unwrap();
}

fn main() {
    let logger = Logger::new();
    let uff_file = UffFile::new(Path::new("../assets/sample_ssd_relu6.uff")).unwrap();
    let engine = create_engine(&logger, &uff_file);

    let mut input_image = image::open("../assets/images/dog.ppm").unwrap().into_rgb();
    let input_buffer = process_input(&input_image);

    let (top_detections, keep_count) = infer(&engine, &input_buffer);
    verify_output(&mut input_image, &top_detections, &keep_count);

    println!("Done!");
}
