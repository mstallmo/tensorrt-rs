mod macros {
    #[macro_export]
    macro_rules! check_cuda {
        ($expression:expr) => {
            unsafe {
                let res = $expression;
                if res != cuda_runtime_sys::cudaError_t::cudaSuccess {
                    let error_message =
                        std::ffi::CStr::from_ptr(cuda_runtime_sys::cudaGetErrorString(res));
                    return Err(anyhow::anyhow!("{}", error_message.to_str().unwrap()));
                }
            }
        };
    }

    #[macro_export]
    macro_rules! binding_func {
        ($function_name:ident<$Trait:path>( $($arg_name:ident : $arg_ty:ty),*) ) => {

            unsafe extern "C" fn $function_name<T: $Trait>(
                context: *mut T,
                $($arg_name: $arg_ty),*
            ) {
                let profiler_ref: &mut T = &mut *context;
                profiler_ref.$function_name($($arg_name),*);
            }
        };
    }
}
