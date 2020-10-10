use std::ffi::CStr;
use std::os::raw::c_char;
use tensorrt_sys::Profiler_t;

pub trait IProfiler {
    fn report_layer_time(&self, layer_name: *const c_char, ms: f32);
}

#[repr(C)]
pub(crate) struct ProfilerBinding<T>
where
    T: IProfiler,
{
    pub report_layer_time: unsafe extern "C" fn(*mut T, *const c_char, f32),
    destroy: unsafe extern "C" fn(*mut Profiler_t, *mut T),
    pub context: *mut T,
}

impl<T> ProfilerBinding<T>
where
    T: IProfiler,
{
    pub fn new(profiler: &mut T) -> Self {
        unsafe extern "C" fn report_layer_time<T>(
            context: *mut T,
            layer_name: *const c_char,
            ms: f32,
        ) where
            T: IProfiler,
        {
            let profiler_ref: &mut T = &mut *context;
            profiler_ref.report_layer_time(layer_name, ms);
        }

        //This is a little un-orthodox but having this extern function allows us to
        //cleanup the memory that we lose when passing the ProfilerBinding as a raw pointer to
        //the C++ bindings. The pointer that gets sent to C++ becomes owned by a C++ object and when
        //that object is destroyed this will get called so we can destroy the memory allocated for
        //the pointer properly.
        unsafe extern "C" fn destroy<T: IProfiler>(binding: *mut Profiler_t, _: *mut T) {
            Box::from_raw(binding as *mut ProfilerBinding<T>);
        }

        let context: *mut T = &mut *profiler;
        ProfilerBinding {
            report_layer_time,
            destroy,
            context,
        }
    }
}

pub struct RustProfiler {}

impl RustProfiler {
    pub fn new() -> Self {
        RustProfiler {}
    }
}

impl IProfiler for RustProfiler {
    fn report_layer_time(&self, layer_name: *const c_char, ms: f32) {
        println!(
            "{} took {} ms",
            unsafe { CStr::from_ptr(layer_name) }.to_str().unwrap(),
            ms
        );
    }
}
