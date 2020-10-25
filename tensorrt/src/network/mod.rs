pub mod layer;

use crate::dims::{Dim, DimsHW};
use crate::engine::DataType;
use layer::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use tensorrt_sys::{
    destroy_network, network_add_activation, network_add_element_wise, network_add_gather,
    network_add_identity_layer, network_add_input, network_add_pooling, network_get_input,
    network_get_layer, network_get_nb_inputs, network_get_nb_layers, network_get_nb_outputs,
    network_get_output, network_mark_output, network_remove_tensor, network_unmark_output,
    nvinfer1_ITensor, tensor_get_name, tensor_set_dimensions,
};

pub struct Network {
    pub(crate) internal_network: *mut tensorrt_sys::Network_t,
}

impl Network {
    pub fn get_nb_inputs(&self) -> i32 {
        unsafe { network_get_nb_inputs(self.internal_network) }
    }

    pub fn add_input<T: Dim>(&self, name: &str, data_type: DataType, dims: T) -> Tensor {
        let internal_tensor = unsafe {
            network_add_input(
                self.internal_network,
                CString::new(name).unwrap().as_ptr(),
                data_type as c_int,
                dims.get_internal_dims(),
            )
        };
        Tensor { internal_tensor }
    }

    pub fn get_input(&self, idx: i32) -> Tensor {
        let internal_tensor = unsafe { network_get_input(self.internal_network, idx) };
        Tensor { internal_tensor }
    }

    pub fn get_nb_layers(&self) -> i32 {
        unsafe { network_get_nb_layers(self.internal_network) }
    }

    pub fn get_layer(&self, index: i32) -> BaseLayer {
        let internal_layer = unsafe { network_get_layer(self.internal_network, index) };
        BaseLayer { internal_layer }
    }

    pub fn get_nb_outputs(&self) -> i32 {
        unsafe { network_get_nb_outputs(self.internal_network) }
    }

    pub fn get_output(&self, index: i32) -> Tensor {
        let internal_tensor = unsafe { network_get_output(self.internal_network, index) };
        Tensor { internal_tensor }
    }

    pub fn remove_tensor(&self, tensor: &Tensor) {
        unsafe { network_remove_tensor(self.internal_network, tensor.internal_tensor) }
    }

    pub fn mark_output(&self, output_tensor: &Tensor) {
        unsafe { network_mark_output(self.internal_network, output_tensor.internal_tensor) }
    }

    pub fn unmark_output(&self, output_tensor: &Tensor) {
        unsafe { network_unmark_output(self.internal_network, output_tensor.internal_tensor) }
    }

    pub fn add_identity_layer(&self, input_tensor: &Tensor) -> IdentityLayer {
        let internal_layer = unsafe {
            network_add_identity_layer(self.internal_network, input_tensor.internal_tensor)
        };
        IdentityLayer { internal_layer }
    }

    pub fn add_element_wise_layer(
        &self,
        input_tensor1: &Tensor,
        input_tensor2: &Tensor,
        op: ElementWiseOperation,
    ) -> ElementWiseLayer {
        let internal_layer = unsafe {
            network_add_element_wise(
                self.internal_network,
                input_tensor1.internal_tensor,
                input_tensor2.internal_tensor,
                op as c_int,
            )
        };
        ElementWiseLayer { internal_layer }
    }

    pub fn add_gather_layer(&self, data: &Tensor, indicies: &Tensor, axis: i32) -> GatherLayer {
        let internal_layer = unsafe {
            network_add_gather(
                self.internal_network,
                data.internal_tensor,
                indicies.internal_tensor,
                axis,
            )
        };
        GatherLayer { internal_layer }
    }

    pub fn add_activation(
        &self,
        input: &Tensor,
        activation_type: ActivationType,
    ) -> ActivationLayer {
        let internal_layer = unsafe {
            network_add_activation(
                self.internal_network,
                input.internal_tensor,
                activation_type as c_int,
            )
        };
        ActivationLayer { internal_layer }
    }

    pub fn add_pooling(
        &self,
        input: &Tensor,
        pooling_type: PoolingType,
        window_size: DimsHW,
    ) -> PoolingLayer {
        let internal_layer = unsafe {
            network_add_pooling(
                self.internal_network,
                input.internal_tensor,
                pooling_type as c_int,
                window_size.internal_dims,
            )
        };
        PoolingLayer { internal_layer }
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe { destroy_network(self.internal_network) };
    }
}

pub struct Tensor {
    pub(crate) internal_tensor: *mut nvinfer1_ITensor,
}

impl Tensor {
    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(tensor_get_name(self.internal_tensor))
                .to_str()
                .unwrap()
                .to_owned()
        }
    }

    pub fn set_dimensions<D: Dim>(&mut self, dims: D) {
        unsafe { tensor_set_dimensions(self.internal_tensor, dims.get_internal_dims()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::dims::{DimsCHW, DimsHW};
    use crate::runtime::Logger;
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use layer::LayerType;
    use lazy_static::lazy_static;
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
        let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
        uff_parser.parse(&uff_file, &network).unwrap();

        network
    }

    #[test]
    fn get_nb_layers_uff() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        assert_eq!(network.get_nb_layers(), 24);
    }

    #[test]
    fn layer_name() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        let layer = network.get_layer(0);
        assert_eq!(layer.get_name(), "wc1");
    }

    #[test]
    fn get_nb_inputs() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        assert_eq!(network.get_nb_inputs(), 1);
    }

    #[test]
    fn add_input() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);

        let tensor = network.add_input("new_input", DataType::Float, DimsCHW::new(1, 28, 28));
        assert_eq!(tensor.get_name(), "new_input");
    }

    #[test]
    fn get_input() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        assert_eq!(network.get_input(0).get_name(), "in");
    }

    #[test]
    fn get_nb_outputs() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        assert_eq!(network.get_nb_outputs(), 1);
    }

    #[test]
    fn get_output() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);

        assert_eq!(network.get_output(0).get_name(), "out");
    }

    #[test]
    fn remove_tensor() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let uff_network = create_network_from_uff(&logger);
        let output_tensor = uff_network.get_layer(21).get_output(0);

        let network = create_network(&logger);
        let tensor = network.add_input("new_input", DataType::Float, DimsCHW::new(1, 28, 28));
        let layer = network.add_identity_layer(&tensor);

        assert_eq!(network.get_layer(0).get_input(0).get_name(), "new_input");
        layer.set_input(0, &output_tensor);
        network.remove_tensor(&tensor);
        assert_eq!(network.get_nb_inputs(), 0);
        assert_eq!(network.get_layer(0).get_input(0).get_name(), "matmul2");
    }

    #[test]
    fn mark_output() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let new_output_tensor = network.get_layer(21).get_output(0);

        assert_eq!(network.get_nb_outputs(), 1);
        network.mark_output(&new_output_tensor);
        assert_eq!(network.get_nb_outputs(), 2);
    }

    #[test]
    fn unmark_output() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network_from_uff(&logger);
        let new_output_tensor = network.get_layer(21).get_output(0);

        assert_eq!(network.get_nb_outputs(), 1);
        network.mark_output(&new_output_tensor);
        assert_eq!(network.get_nb_outputs(), 2);
        network.unmark_output(&new_output_tensor);
        assert_eq!(network.get_nb_outputs(), 1);
    }

    #[test]
    fn add_identity_layer() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let tensor = network.add_input("new_input", DataType::Float, DimsCHW::new(1, 28, 28));
        network.add_identity_layer(&tensor);
        assert_eq!(network.get_nb_layers(), 1);
    }

    #[test]
    fn add_element_wise_layer() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let input2 = network.add_input("new_input2", DataType::Float, DimsCHW::new(1, 28, 28));
        network.add_element_wise_layer(&input1, &input2, ElementWiseOperation::Sum);

        assert_eq!(network.get_nb_layers(), 1);
        assert_eq!(network.get_layer(0).get_type(), LayerType::ElementWise);
    }

    #[test]
    fn add_gather_layer() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let input2 = network.add_input("new_input2", DataType::Float, DimsCHW::new(1, 28, 28));
        network.add_gather_layer(&input1, &input2, 1);

        assert_eq!(network.get_nb_layers(), 1);
        assert_eq!(network.get_layer(0).get_type(), LayerType::Gather);
    }

    #[test]
    fn add_activation() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        network.add_activation(&input1, ActivationType::Relu);

        assert_eq!(network.get_layer(0).get_type(), LayerType::Activation);
    }

    #[test]
    fn add_pooling() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));

        network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));
        assert_eq!(network.get_layer(0).get_type(), LayerType::Pooling);
    }
}
