use crate::binding_func;
use std::ffi::CStr;
use std::os::raw::c_char;
use tensorrt_sys::{create_profiler, destroy_profiler, CppProfiler, Profiler_t};

pub trait IProfiler {
    fn report_layer_time(&self, layer_name: *const c_char, ms: f32);
}

pub struct Profiler<P: IProfiler> {
    pub(crate) internal_profiler: *mut CppProfiler,
    _supplied_profiler: P,
}

impl<P: IProfiler> Profiler<P> {
    pub fn new(mut rust_profiler: P) -> Self {
        let profiler_ptr =
            Box::into_raw(Box::new(ProfilerBinding::new(&mut rust_profiler))) as *mut Profiler_t;
        let internal_profiler = unsafe { create_profiler(profiler_ptr) };

        Profiler {
            internal_profiler,
            _supplied_profiler: rust_profiler,
        }
    }
}

impl<P: IProfiler> Drop for Profiler<P> {
    fn drop(&mut self) {
        unsafe {
            destroy_profiler(self.internal_profiler);
        }
    }
}

pub struct DefaultProfiler {}

impl DefaultProfiler {
    pub fn new() -> Self {
        DefaultProfiler {}
    }
}

impl IProfiler for DefaultProfiler {
    fn report_layer_time(&self, layer_name: *const c_char, ms: f32) {
        println!(
            "{} took {} ms",
            unsafe { CStr::from_ptr(layer_name) }.to_str().unwrap(),
            ms
        );
    }
}

#[repr(C)]
struct ProfilerBinding<T>
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
        binding_func!(report_layer_time<IProfiler>(layer_name: *const c_char, ms: f32));

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
