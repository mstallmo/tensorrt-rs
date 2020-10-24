use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::{elementwise_destroy, elementwise_get_operation, elementwise_set_operation};

#[repr(C)]
#[derive(Debug, FromPrimitive, Eq, PartialEq)]
pub enum ElementWiseOperation {
    Sum,
    Prod,
    Max,
    Min,
    Sub,
    Div,
    Pow,
}

#[derive(Layer)]
pub struct ElementWiseLayer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}

impl ElementWiseLayer {
    pub fn get_operation(&self) -> ElementWiseOperation {
        let raw = unsafe { elementwise_get_operation(self.internal_layer) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    pub fn set_operation(&self, op: ElementWiseOperation) {
        unsafe { elementwise_set_operation(self.internal_layer, op as c_int) }
    }
}

impl Drop for ElementWiseLayer {
    fn drop(&mut self) {
        unsafe { elementwise_destroy(self.internal_layer) }
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
    fn get_operation() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(1, 1));
        let input2 = network.add_input("new_input2", DataType::Float, DimsHW::new(1, 1));
        let element_wise_layer =
            network.add_element_wise_layer(&input1, &input2, ElementWiseOperation::Sum);

        assert_eq!(
            element_wise_layer.get_operation(),
            ElementWiseOperation::Sum
        );
    }

    #[test]
    fn set_operation() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(1, 1));
        let input2 = network.add_input("new_input2", DataType::Float, DimsHW::new(1, 1));
        let element_wise_layer =
            network.add_element_wise_layer(&input1, &input2, ElementWiseOperation::Sum);
        element_wise_layer.set_operation(ElementWiseOperation::Prod);

        assert_eq!(
            element_wise_layer.get_operation(),
            ElementWiseOperation::Prod
        );
    }
}
