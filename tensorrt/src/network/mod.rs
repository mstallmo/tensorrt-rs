pub mod layer;

use crate::dims::IsDim;
use layer::Layer;
use tensorrt_sys::{destroy_network, network_get_input, network_get_layer, tensor_set_dimensions};

pub struct Network {
    pub(crate) internal_network: *mut tensorrt_sys::Network_t,
}

pub struct Tensor {
    pub(crate) internal_tensor: *mut tensorrt_sys::Tensor_t,
}

impl Network {
    pub fn get_input(&self, idx: i32) -> Tensor {
        let internal_tensor = unsafe { network_get_input(self.internal_network, idx) };
        Tensor { internal_tensor }
    }

    pub fn get_layer(&self, index: i32) -> Layer {
        let internal_layer = unsafe { network_get_layer(self.internal_network, index) };
        Layer { internal_layer }
    }
}

impl Tensor {
    pub fn set_dimensions<D: IsDim>(&mut self, dims: D) {
        unsafe { tensor_set_dimensions(self.internal_tensor, dims.internal_dims()) };
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe { destroy_network(self.internal_network) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use crate::dims::DimsCHW;
    use crate::runtime::Logger;
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use lazy_static::lazy_static;
    use std::path::Path;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    fn create_network(logger: &Logger) -> Network {
        let builder = Builder::new(&logger);
        let network = builder.create_network();

        let uff_parser = UffParser::new();
        let dim = DimsCHW::new(1, 28, 28);

        uff_parser
            .register_input("in", dim, UffInputOrder::Nchw)
            .unwrap();
        uff_parser.register_output("out").unwrap();
        let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
        uff_parser.parse(&uff_file, &network).unwrap();

        network
    }

    #[test]
    fn layer_name() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let layer = network.get_layer(0);

        assert_eq!(layer.get_name(), "wc1");
    }
}
