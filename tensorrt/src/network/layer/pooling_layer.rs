use super::*;
use crate::dims::{Dim, Dims, DimsHW};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::{
    nvinfer1_IPoolingLayer, pooling_get_average_count_excludes_padding, pooling_get_blend_factor,
    pooling_get_padding, pooling_get_padding_mode, pooling_get_pooling_type,
    pooling_get_post_padding, pooling_get_pre_padding, pooling_get_stride, pooling_get_window_size,
    pooling_set_average_count_excludes_padding, pooling_set_blend_factor, pooling_set_padding,
    pooling_set_padding_mode, pooling_set_pooling_type, pooling_set_post_padding,
    pooling_set_pre_padding, pooling_set_stride, pooling_set_window_size,
};

#[repr(C)]
#[derive(FromPrimitive, Debug, Eq, PartialEq)]
pub enum PoolingType {
    Max,
    Average,
    MaxAverageBlend,
}

#[repr(C)]
#[derive(FromPrimitive, Debug, Eq, PartialEq)]
pub enum PaddingMode {
    ExplicitRoundDown,
    ExplicitRoundUp,
    SameUpper,
    SameLower,
    CaffeRoundDown,
    CaffeRoundUp,
}

#[derive(Layer)]
pub struct PoolingLayer {
    pub(crate) internal_layer: *mut nvinfer1_IPoolingLayer,
}

impl PoolingLayer {
    pub fn get_pooling_type(&self) -> PoolingType {
        let raw = unsafe { pooling_get_pooling_type(self.internal_layer) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    pub fn set_pooling_type(&self, pooling_type: PoolingType) {
        unsafe { pooling_set_pooling_type(self.internal_layer, pooling_type as i32) }
    }

    pub fn get_window_size(&self) -> DimsHW {
        let raw = unsafe { pooling_get_window_size(self.internal_layer) };
        DimsHW(raw)
    }

    pub fn set_window_size(&self, dims: DimsHW) {
        unsafe { pooling_set_window_size(self.internal_layer, dims.0) }
    }

    pub fn get_stride(&self) -> DimsHW {
        let raw = unsafe { pooling_get_stride(self.internal_layer) };
        DimsHW(raw)
    }

    pub fn set_stride(&self, dims: DimsHW) {
        unsafe { pooling_set_stride(self.internal_layer, dims.0) }
    }

    pub fn get_padding(&self) -> DimsHW {
        let raw = unsafe { pooling_get_padding(self.internal_layer) };
        DimsHW(raw)
    }

    pub fn set_padding(&self, padding: DimsHW) {
        unsafe { pooling_set_padding(self.internal_layer, padding.0) }
    }

    pub fn get_blend_factor(&self) -> f32 {
        unsafe { pooling_get_blend_factor(self.internal_layer) }
    }

    pub fn set_blend_factor(&self, factor: f32) {
        unsafe { pooling_set_blend_factor(self.internal_layer, factor) }
    }

    pub fn get_average_count_excludes_padding(&self) -> bool {
        unsafe { pooling_get_average_count_excludes_padding(self.internal_layer) }
    }

    pub fn set_average_count_excludes_padding(&self, exclusive: bool) {
        unsafe { pooling_set_average_count_excludes_padding(self.internal_layer, exclusive) }
    }

    pub fn get_pre_padding(&self) -> Dims {
        let raw = unsafe { pooling_get_pre_padding(self.internal_layer) };
        Dims(raw)
    }

    pub fn set_pre_padding<T: Dim>(&self, padding: T) {
        unsafe { pooling_set_pre_padding(self.internal_layer, padding.get_internal_dims()) }
    }

    pub fn get_post_padding(&self) -> Dims {
        let raw = unsafe { pooling_get_post_padding(self.internal_layer) };
        Dims(raw)
    }

    pub fn set_post_padding<T: Dim>(&self, padding: T) {
        unsafe { pooling_set_post_padding(self.internal_layer, padding.get_internal_dims()) }
    }

    pub fn get_padding_mode(&self) -> PaddingMode {
        let raw = unsafe { pooling_get_padding_mode(self.internal_layer) };
        FromPrimitive::from_i32(raw).unwrap()
    }

    pub fn set_padding_mode(&self, mode: PaddingMode) {
        unsafe { pooling_set_padding_mode(self.internal_layer, mode as i32) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::dims::{Dim, DimsCHW, DimsHW};
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
    fn get_pooling_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        assert_eq!(pooling.get_pooling_type(), PoolingType::Max);
    }

    #[test]
    fn set_pooling_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        pooling.set_pooling_type(PoolingType::Average);

        assert_eq!(pooling.get_pooling_type(), PoolingType::Average);
    }

    #[test]
    fn get_window_size() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        let dims = pooling.get_window_size();
        assert_eq!(dims.d()[0], 10);
        assert_eq!(dims.d()[1], 10);
    }

    #[test]
    fn set_window_size() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        pooling.set_window_size(DimsHW::new(20, 20));

        let dims = pooling.get_window_size();
        assert_eq!(dims.d()[0], 20);
        assert_eq!(dims.d()[1], 20);
    }

    #[test]
    fn get_stride() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        let stride = pooling.get_stride();
        assert_eq!(stride.d()[0], 10);
        assert_eq!(stride.d()[1], 10);
    }

    #[test]
    fn set_stride() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        pooling.set_stride(DimsHW::new(20, 20));
        let stride = pooling.get_stride();

        assert_eq!(stride.d()[0], 20);
        assert_eq!(stride.d()[1], 20);
    }

    #[test]
    fn get_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        let padding = pooling.get_padding();
        assert_eq!(padding.d()[0], 0);
        assert_eq!(padding.d()[1], 0);
    }

    #[test]
    fn set_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Max, DimsHW::new(10, 10));

        pooling.set_padding(DimsHW::new(0, 10));
        let padding = pooling.get_padding();
        assert_eq!(padding.d()[0], 0);
        assert_eq!(padding.d()[1], 10);
    }

    #[test]
    fn get_blend_factor() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling =
            network.add_pooling(&input1, PoolingType::MaxAverageBlend, DimsHW::new(10, 10));

        assert_eq!(pooling.get_blend_factor(), 0.0);
    }

    #[test]
    fn set_blend_factor() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling =
            network.add_pooling(&input1, PoolingType::MaxAverageBlend, DimsHW::new(10, 10));

        pooling.set_blend_factor(0.5);
        assert_eq!(pooling.get_blend_factor(), 0.5);
    }

    #[test]
    fn get_average_count_excludes_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        assert_eq!(pooling.get_average_count_excludes_padding(), true);
    }

    #[test]
    fn set_average_count_excludes_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        pooling.set_average_count_excludes_padding(false);
        assert_eq!(pooling.get_average_count_excludes_padding(), false);
    }

    #[test]
    fn get_pre_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        let padding = pooling.get_pre_padding();
        assert_eq!(padding.d()[0], 0);
        assert_eq!(padding.d()[1], 0);
        assert_eq!(padding.nb_dims(), 2);
    }

    #[test]
    fn set_pre_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        pooling.set_pre_padding(DimsHW::new(10, 10));
        let padding = pooling.get_pre_padding();
        assert_eq!(padding.d()[0], 10);
        assert_eq!(padding.d()[1], 10);
        assert_eq!(padding.nb_dims(), 2);
    }

    #[test]
    fn get_post_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        let padding = pooling.get_post_padding();
        assert_eq!(padding.d()[0], 0);
        assert_eq!(padding.d()[1], 0);
        assert_eq!(padding.nb_dims(), 2);
    }

    #[test]
    fn set_post_padding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        pooling.set_post_padding(DimsHW::new(10, 10));
        let padding = pooling.get_post_padding();
        assert_eq!(padding.d()[0], 10);
        assert_eq!(padding.d()[1], 10);
        assert_eq!(padding.nb_dims(), 2);
    }

    #[test]
    fn get_padding_mode() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        assert_eq!(pooling.get_padding_mode(), PaddingMode::ExplicitRoundDown);
    }

    #[test]
    fn set_padding_mode() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let network = create_network(&logger);
        let input1 = network.add_input("new_input1", DataType::Float, DimsCHW::new(1, 28, 28));
        let pooling = network.add_pooling(&input1, PoolingType::Average, DimsHW::new(10, 10));

        pooling.set_padding_mode(PaddingMode::ExplicitRoundUp);
        assert_eq!(pooling.get_padding_mode(), PaddingMode::ExplicitRoundUp);
    }
}
