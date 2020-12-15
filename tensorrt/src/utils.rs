pub mod cuda_utils {
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
}
