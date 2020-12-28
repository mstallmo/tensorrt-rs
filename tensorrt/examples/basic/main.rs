use std::path::Path;
use tensorrt_rs::builder::{Builder, NetworkBuildFlags};
use tensorrt_rs::dims::DimsCHW;
use tensorrt_rs::engine::Engine;
use tensorrt_rs::runtime::Logger;
use tensorrt_rs::uff::{UffFile, UffInputOrder, UffParser};

fn create_engine(logger: &Logger, uff_file: UffFile) -> Engine {
    let builder = Builder::new(&logger);
    let network = builder.create_network_v2(NetworkBuildFlags::DEFAULT);

    let uff_parser = UffParser::new();
    let dim = DimsCHW::new(1, 28, 28);
    uff_parser
        .register_input("in", dim, UffInputOrder::Nchw)
        .unwrap();
    uff_parser.register_output("out").unwrap();
    uff_parser.parse(&uff_file, &network).unwrap();

    builder.build_cuda_engine(&network)
}

fn main() {
    let logger = Logger::new();
    let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
    let engine = create_engine(&logger, uff_file);

    println!("Engine number of bindings: {}", engine.get_nb_bindings());

    for binding_index in 0..engine.get_nb_bindings() {
        println!(
            "Binding name at {}: {}",
            binding_index,
            engine.get_binding_name(binding_index).unwrap()
        );
    }
}
