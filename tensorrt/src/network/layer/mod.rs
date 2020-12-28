pub use activation_layer::{ActivationLayer, ActivationType};
pub use element_wise_layer::{ElementWiseLayer, ElementWiseOperation};
pub use gather_layer::GatherLayer;
pub use identity_layer::IdentityLayer;
pub use pooling_layer::{PaddingMode, PoolingLayer, PoolingType};

mod activation_layer;
mod element_wise_layer;
mod gather_layer;
mod identity_layer;
mod pooling_layer;

use crate::engine::DataType;
use crate::network::Tensor;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::{
    layer_get_input, layer_get_name, layer_get_nb_inputs, layer_get_nb_outputs, layer_get_output,
    layer_get_output_type, layer_get_precision, layer_get_type, layer_output_type_is_set,
    layer_precision_is_set, layer_reset_output_type, layer_reset_precision, layer_set_input,
    layer_set_name, layer_set_output_type, layer_set_precision, nvinfer1_ILayer,
};

#[repr(C)]
#[derive(Debug, FromPrimitive, Eq, PartialEq)]
pub enum LayerType {
    Convolution,
    FullyConnected,
    Activation,
    Pooling,
    LRN,
    Scale,
    SoftMax,
    DeConvolution,
    Concatenation,
    ElementWise,
    Plugin,
    Rnn,
    Unary,
    Padding,
    Shuffle,
    Reduce,
    TopK,
    Gather,
    MatrixMultiply,
    RaggedSoftMax,
    Constant,
    RnnV2,
    Identity,
    PluginV2,
    Slice,
}

pub trait Layer: private::LayerPrivate {
    fn get_type(&self) -> LayerType {
        let raw = unsafe { layer_get_type(self.get_internal_layer()) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    fn set_name(&self, name: &str) {
        unsafe {
            layer_set_name(
                self.get_internal_layer(),
                CString::new(name).unwrap().as_ptr(),
            )
        }
    }

    fn get_name(&self) -> String {
        let raw_string = unsafe {
            let ptr = layer_get_name(self.get_internal_layer());
            CStr::from_ptr(ptr)
        };

        raw_string.to_str().unwrap().to_string()
    }

    fn get_nb_inputs(&self) -> i32 {
        unsafe { layer_get_nb_inputs(self.get_internal_layer()) }
    }

    fn get_input(&self, index: i32) -> Tensor {
        let internal_tensor = unsafe { layer_get_input(self.get_internal_layer(), index) };
        Tensor { internal_tensor }
    }

    fn set_input(&self, index: i32, tensor: &Tensor) {
        unsafe { layer_set_input(self.get_internal_layer(), index, tensor.internal_tensor) }
    }

    fn get_nb_outputs(&self) -> i32 {
        unsafe { layer_get_nb_outputs(self.get_internal_layer()) }
    }

    fn get_output(&self, index: i32) -> Tensor {
        let internal_tensor = unsafe { layer_get_output(self.get_internal_layer(), index) };
        Tensor { internal_tensor }
    }

    fn set_precision(&self, precision: DataType) {
        unsafe { layer_set_precision(self.get_internal_layer(), precision as c_int) }
    }

    fn get_precision(&self) -> DataType {
        let raw = unsafe { layer_get_precision(self.get_internal_layer()) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    fn precision_is_set(&self) -> bool {
        unsafe { layer_precision_is_set(self.get_internal_layer()) }
    }

    fn reset_precision(&self) {
        unsafe { layer_reset_precision(self.get_internal_layer()) }
    }

    fn set_output_type(&self, index: i32, data_type: DataType) {
        unsafe { layer_set_output_type(self.get_internal_layer(), index, data_type as c_int) }
    }

    fn get_output_type(&self, index: i32) -> DataType {
        let raw = unsafe { layer_get_output_type(self.get_internal_layer(), index) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    fn output_type_is_set(&self, index: i32) -> bool {
        unsafe { layer_output_type_is_set(self.get_internal_layer(), index) }
    }

    fn rest_output_type(&self, index: i32) {
        unsafe { layer_reset_output_type(self.get_internal_layer(), index) }
    }
}

mod private {
    use tensorrt_sys::nvinfer1_ILayer;

    pub trait LayerPrivate {
        fn get_internal_layer(&self) -> *mut nvinfer1_ILayer;
    }
}

#[derive(Layer)]
pub struct BaseLayer {
    pub(crate) internal_layer: *mut nvinfer1_ILayer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{Builder, NetworkBuildFlags};
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
        builder.create_network_v2(NetworkBuildFlags::DEFAULT)
    }

    fn create_network_from_uff(logger: &Logger) -> Network {
        let builder = Builder::new(&logger);
        let network = builder.create_network_v2(NetworkBuildFlags::DEFAULT);

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
    fn get_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        let layer = network.get_layer(1);
        assert_eq!(layer.get_type(), LayerType::Convolution);
    }

    #[test]
    fn set_name() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        let layer = network.get_layer(1);
        assert_eq!(layer.get_name(), "conv1");
        layer.set_name("first_conv");
        assert_eq!(layer.get_name(), "first_conv");
    }

    #[test]
    fn get_name() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        let layer = network.get_layer(1);
        assert_eq!(layer.get_name(), "conv1");
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
    fn get_nb_outputs() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        assert_eq!(layer.get_nb_outputs(), 1);
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

    #[test]
    fn get_precision() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        assert_eq!(layer.get_precision(), DataType::Float);
    }

    #[test]
    fn set_precision() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        layer.set_precision(DataType::Half);
        assert_eq!(layer.get_precision(), DataType::Half);
    }

    #[test]
    fn precision_is_set_true() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);
        layer.set_precision(DataType::Half);

        assert_eq!(layer.precision_is_set(), true);
    }

    #[test]
    fn precision_is_set_false() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        assert_eq!(layer.precision_is_set(), false);
    }

    #[test]
    fn reset_precision() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        layer.set_precision(DataType::Half);
        assert_eq!(layer.precision_is_set(), true);
        layer.reset_precision();
        assert_eq!(layer.precision_is_set(), false);
    }

    #[test]
    fn set_output_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        layer.set_output_type(0, DataType::Half);
        assert_eq!(layer.get_output_type(0), DataType::Half);
    }

    #[test]
    fn output_type_is_set() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        layer.set_output_type(0, DataType::Half);
        assert_eq!(layer.output_type_is_set(0), true);
    }

    #[test]
    fn rest_output_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let layer = network.get_layer(1);

        layer.set_output_type(0, DataType::Half);
        assert_eq!(layer.output_type_is_set(0), true);
        layer.rest_output_type(0);
        assert_eq!(layer.output_type_is_set(0), false);
    }
}
