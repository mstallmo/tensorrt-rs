use super::*;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::{
    gather_layer_get_gather_axis, gather_layer_set_gather_axis, nvinfer1_IGatherLayer,
};

#[derive(Layer)]
pub struct GatherLayer {
    pub(crate) internal_layer: *mut nvinfer1_IGatherLayer,
}

impl GatherLayer {
    pub fn get_gather_axis(&self) -> i32 {
        unsafe { gather_layer_get_gather_axis(self.internal_layer) }
    }

    pub fn set_gather_axis(&self, axis: i32) {
        unsafe { gather_layer_set_gather_axis(self.internal_layer, axis) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::dims::DimsHW;
    use crate::network::Network;
    use crate::runtime::Logger;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    fn create_network(logger: &Logger) -> Network {
        let builder = Builder::new(logger);
        builder.create_network_v2(NetworkBuildFlags::EXPLICIT_BATCH)
    }

    #[test]
    fn get_gather_axis() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let input2 = network.add_input("new_input2", DataType::Float, DimsHW::new(10, 10));
        let gather_layer = network.add_gather_layer(&input1, &input2, 1);

        assert_eq!(gather_layer.get_gather_axis(), 1);
    }

    #[test]
    fn set_gather_axis() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let input2 = network.add_input("new_input2", DataType::Float, DimsHW::new(10, 10));
        let gather_layer = network.add_gather_layer(&input1, &input2, 1);

        gather_layer.set_gather_axis(0);
        assert_eq!(gather_layer.get_gather_axis(), 0);
    }
}
