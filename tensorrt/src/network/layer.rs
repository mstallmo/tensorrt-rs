use crate::network::Tensor;
use std::ffi::CStr;
use tensorrt_sys::{
    layer_get_input, layer_get_name, layer_get_nb_inputs, layer_get_output, layer_set_input,
};

pub struct Layer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}

impl Layer {
    pub fn get_name(&self) -> String {
        let raw_string = unsafe {
            let ptr = layer_get_name(self.internal_layer);
            CStr::from_ptr(ptr)
        };

        raw_string.to_str().unwrap().to_string()
    }

    pub fn get_nb_inputs(&self) -> i32 {
        unsafe { layer_get_nb_inputs(self.internal_layer) }
    }

    pub fn get_input(&self, index: i32) -> Tensor {
        let internal_tensor = unsafe { layer_get_input(self.internal_layer, index) };
        Tensor { internal_tensor }
    }

    pub fn set_input(&self, index: i32, tensor: &Tensor) {
        unsafe { layer_set_input(self.internal_layer, index, tensor.internal_tensor) }
    }

    pub fn get_output(&self, index: i32) -> Tensor {
        let internal_tensor = unsafe { layer_get_output(self.internal_layer, index) };
        Tensor { internal_tensor }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::dims::DimsCHW;
    use crate::engine::DataType;
    use crate::network::Network;
    use crate::runtime::Logger;
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use lazy_static::lazy_static;
    use std::env::current_dir;
    use std::path::Path;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    fn create_network(logger: &Logger) -> Network {
        let builder = Builder::new(logger);
        builder.create_network()
    }

    fn create_network_from_uff(logger: &Logger) -> Network {
        let builder = Builder::new(&logger);
        let network = builder.create_network();

        let uff_parser = UffParser::new();
        let dim = DimsCHW::new(1, 28, 28);

        uff_parser
            .register_input("in", dim, UffInputOrder::Nchw)
            .unwrap();
        uff_parser.register_output("out").unwrap();
        println!("{}", current_dir().unwrap().display());
        let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
        uff_parser.parse(&uff_file, &network).unwrap();

        network
    }

    #[test]
    fn get_nb_inputs() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        let layer = network.get_layer(1);

        assert_eq!(layer.get_nb_inputs(), 1);
    }

    #[test]
    fn get_output() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        assert_eq!(layer.get_output(0).get_name(), "conv1");
    }

    #[test]
    fn set_input() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let uff_network = create_network_from_uff(&logger);
        let output_tensor = uff_network.get_layer(21).get_output(0);

        let network = create_network(&logger);
        let tensor = network.add_input("new_input", DataType::Float, DimsCHW::new(1, 28, 28));
        let layer = network.add_identity_layer(&tensor);

        assert_eq!(layer.get_input(0).get_name(), "new_input");
        layer.set_input(0, &output_tensor);
        assert_eq!(layer.get_input(0).get_name(), "matmul2");
    }
}
