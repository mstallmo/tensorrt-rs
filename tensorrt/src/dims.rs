use std::error;
use std::fmt::Formatter;
use std::os::raw::{c_int, c_uint};
use tensorrt_sys::{
    create_dims, create_dims2, create_dims3, create_dims4, create_dimsCHW, create_dimsHW,
    create_dimsNCHW, destroy_dims, dims2_set_dimension_types, dims3_set_dimension_types,
    dims4_set_dimension_types, Dims_t,
};

pub trait IsDim {
    fn internal_dims(&self) -> *mut Dims_t;
}

#[repr(C)]
pub enum DimensionType {
    Spacial,
    Channel,
    Index,
    Sequence,
}

pub struct Dims {
    pub(crate) internal_dims: *mut Dims_t,
}

impl Dims {
    pub fn new(
        num_dims: i32,
        dimension_sizes: &mut [i32; 8],
        dimension_types: &mut [DimensionType; 8],
    ) -> Dims {
        let internal_dims = unsafe {
            create_dims(
                num_dims,
                dimension_sizes.as_mut_ptr() as *mut c_int,
                dimension_types.as_ptr() as *const c_uint,
            )
        };

        Dims { internal_dims }
    }
}

impl Drop for Dims {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) };
    }
}

//TODO: Make IsDim a derive proc macro
impl IsDim for Dims {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct Dims2 {
    pub(crate) internal_dims: *mut Dims_t,
}

impl Dims2 {
    pub fn new(dim1: i32, dim2: i32) -> Dims2 {
        let internal_dims = unsafe { create_dims2(dim1, dim2) };

        Dims2 { internal_dims }
    }

    pub fn set_dimension_types(&self, type1: DimensionType, type2: DimensionType) {
        unsafe { dims2_set_dimension_types(self.internal_dims, type1 as u32, type2 as u32) }
    }
}

impl Drop for Dims2 {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) };
    }
}

impl IsDim for Dims2 {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct DimsHW {
    pub(crate) internal_dims: *mut Dims_t,
}

impl DimsHW {
    pub fn new(height: i32, width: i32) -> DimsHW {
        let internal_dims = unsafe { create_dimsHW(height, width) };

        DimsHW { internal_dims }
    }
}

impl IsDim for DimsHW {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct Dims3 {
    pub(crate) internal_dims: *mut Dims_t,
}

impl Dims3 {
    pub fn new(dim1: i32, dim2: i32, dim3: i32) -> Dims3 {
        let internal_dims = unsafe { create_dims3(dim1, dim2, dim3) };
        Dims3 { internal_dims }
    }

    pub fn set_dimension_types(
        &mut self,
        type1: DimensionType,
        type2: DimensionType,
        type3: DimensionType,
    ) {
        unsafe {
            dims3_set_dimension_types(self.internal_dims, type1 as u32, type2 as u32, type3 as u32)
        };
    }
}

impl Drop for Dims3 {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) };
    }
}

impl IsDim for Dims3 {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct DimsCHW {
    pub(crate) internal_dims: *mut Dims_t,
}

impl DimsCHW {
    pub fn new(channels: i32, height: i32, width: i32) -> DimsCHW {
        let internal_dims = unsafe { create_dimsCHW(channels, height, width) };
        DimsCHW { internal_dims }
    }
}

impl Drop for DimsCHW {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) };
    }
}

impl IsDim for DimsCHW {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct Dims4 {
    pub(crate) internal_dims: *mut Dims_t,
}

impl Dims4 {
    pub fn new(dim1: i32, dim2: i32, dim3: i32, dim4: i32) -> Dims4 {
        let internal_dims = unsafe { create_dims4(dim1, dim2, dim3, dim4) };
        Dims4 { internal_dims }
    }

    pub fn set_dimension_types(
        &mut self,
        type1: DimensionType,
        type2: DimensionType,
        type3: DimensionType,
        type4: DimensionType,
    ) {
        unsafe {
            dims4_set_dimension_types(
                self.internal_dims,
                type1 as u32,
                type2 as u32,
                type3 as u32,
                type4 as u32,
            )
        };
    }
}

impl Drop for Dims4 {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) };
    }
}

impl IsDim for Dims4 {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
    }
}

pub struct DimsNCHW {
    pub(crate) internal_dims: *mut Dims_t,
}

impl DimsNCHW {
    pub fn new(index: i32, channels: i32, height: i32, width: i32) -> DimsNCHW {
        let internal_dims = unsafe { create_dimsNCHW(index, channels, height, width) };
        DimsNCHW { internal_dims }
    }
}

impl Drop for DimsNCHW {
    fn drop(&mut self) {
        unsafe { destroy_dims(self.internal_dims) }
    }
}

impl IsDim for DimsNCHW {
    fn internal_dims(&self) -> *mut Dims_t {
        self.internal_dims
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
