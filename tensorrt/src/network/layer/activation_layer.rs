use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::{
    activation_get_activation_type, activation_get_alpha, activation_get_beta,
    activation_set_activation_type, activation_set_alpha, activation_set_beta,
    nvinfer1_IActivationLayer,
};

#[repr(C)]
#[derive(Debug, Eq, PartialEq, FromPrimitive)]
pub enum ActivationType {
    Relu,
    Sigmoid,
    Tanh,
    LeakyRelu,
    Elu,
    Selu,
    SoftSign,
    SoftPlus,
    Clip,
    HardSigmoid,
    ScaledTanh,
    ThresholdedRelu,
}

#[derive(Layer)]
pub struct ActivationLayer {
    pub(crate) internal_layer: *mut nvinfer1_IActivationLayer,
}

impl ActivationLayer {
    pub fn get_activation_type(&self) -> ActivationType {
        let raw = unsafe { activation_get_activation_type(self.internal_layer) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    pub fn set_activation_type(&self, activation_type: ActivationType) {
        unsafe { activation_set_activation_type(self.internal_layer, activation_type as c_int) }
    }

    pub fn get_alpha(&self) -> f32 {
        unsafe { activation_get_alpha(self.internal_layer) }
    }

    pub fn set_alpha(&self, alpha: f32) {
        unsafe { activation_set_alpha(self.internal_layer, alpha) }
    }

    pub fn get_beta(&self) -> f32 {
        unsafe { activation_get_beta(self.internal_layer) }
    }

    pub fn set_beta(&self, beta: f32) {
        unsafe { activation_set_beta(self.internal_layer, beta) }
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
    fn get_activation_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let activation_layer = network.add_activation(&input1, ActivationType::Relu);

        assert_eq!(activation_layer.get_activation_type(), ActivationType::Relu);
    }

    #[test]
    fn set_activation_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let activation_layer = network.add_activation(&input1, ActivationType::Relu);

        activation_layer.set_activation_type(ActivationType::Sigmoid);
        assert_eq!(
            activation_layer.get_activation_type(),
            ActivationType::Sigmoid
        );
    }

    #[test]
    fn set_alpha() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let activation_layer = network.add_activation(&input1, ActivationType::Relu);

        activation_layer.set_alpha(1.0);
        assert_eq!(activation_layer.get_alpha(), 1.0);
    }

    #[test]
    fn set_beta() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let input1 = network.add_input("new_input1", DataType::Float, DimsHW::new(10, 10));
        let activation_layer = network.add_activation(&input1, ActivationType::Relu);

        activation_layer.set_beta(2.0);
        assert_eq!(activation_layer.get_beta(), 2.0);
    }
}
