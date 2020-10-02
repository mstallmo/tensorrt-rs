use cmake::Config;

fn cuda_configuration() {
    let cudadir = if !env!("CUDA_INSTALL_DIR").is_empty() {
        env!("CUDA_INSTALL_DIR")
    } else {
        "/usr/local/cuda"
    };

    println!("cargo:rustc-link-search={}/lib64", cudadir);
    println!("cargo:rustc-link-lib=dylib=cudart");
}

fn tensorrt_configuration() {
    println!("cargo:rustc-link-lib=dylib=nvinfer");
    if !env!("TRT_LIB_DIR").is_empty() {
        println!("cargo:rustc-link-search={}", env!("TRT_LIB_DIR"));
    }
    println!("cargo:rustc-link-lib=dylib=nvinfer");
    println!("cargo:rustc-link-lib=dylib=nvonnxparser");
    println!("cargo:rustc-link-lib=dylib=nvparsers");
    println!("cargo:rustc-link-lib=dylib=nvinfer_plugin");
}

// Not sure if I love this solution but I think it's relatively robust enough for now on Unix systems.
// Still have to thoroughly test what happens with a TRT library installed that's not done by the
// dpkg. It's possible that we'll just have to fall back to only supporting one system library and assuming that
// the user has the correct library installed and is viewable via ldconfig.
//
// Hopefully something like this will work for Windows installs as well, not having a default library
// install location will make that significantly harder.
fn main() {
    let dst = Config::new("trt-sys").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=trt-sys");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    tensorrt_configuration();
    cuda_configuration();
}
