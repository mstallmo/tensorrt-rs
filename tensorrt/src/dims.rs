use std::error;
use std::fmt::Formatter;
use std::os::raw::c_int;
use std::vec::Vec;
use tensorrt_sys;

#[repr(C)]
pub enum DimensionType {
    Spacial,
    Channel,
    Index,
    Sequence,
}

pub struct Dims {
    pub(crate) internal_dims: tensorrt_sys::Dims,
    _dimension_sizes: Vec<i32>,
    _dimension_types: Vec<DimensionType>,
}

impl Dims {
    pub fn new(
        num_dims: i32,
        mut dimension_sizes: Vec<i32>,
        mut dimension_types: Vec<DimensionType>,
    ) -> Result<Dims, DimsShapeError> {
        if dimension_sizes.len() != dimension_types.len() {
            return Err(DimsShapeError::new("Shape of sizes and types do not match"));
        }

        Ok(Dims {
            internal_dims: tensorrt_sys::Dims {
                nbDims: num_dims,
                d: dimension_sizes.as_mut_ptr(),
                type_: dimension_types.as_mut_ptr() as *mut i32,
            },
            _dimension_sizes: dimension_sizes,
            _dimension_types: dimension_types,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DimsShapeError {
    message: String,
}

impl DimsShapeError {
    pub fn new(message: &str) -> Self {
        DimsShapeError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for DimsShapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for DimsShapeError {}
