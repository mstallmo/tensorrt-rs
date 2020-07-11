use cmake::Config;

fn main() {
    let dst = Config::new("trt-sys")
                     .generator("Ninja")
                     .build();

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-flags=-l trt-sys");
//    println!("cargo:rustc-link-lib=trt-sys");
//    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-link-search=D:/projects/TensorRT-7.1.3.4/lib");
    println!("cargo:rustc-flags=-l dylib=nvinfer");
    println!("cargo:rustc-flags=-l dylib=nvinfer_plugin");
    println!("cargo:rustc-flags=-l dylib=nvonnxparser");
    println!("cargo:rustc-flags=-l dylib=nvparsers");
    println!("cargo:rustc-flags=-l dylib=myelin64_1");
//    println!("cargo:rustc-flags=-L /usr/local/cuda/lib64");
    println!(r#"cargo:rustc-link-search=C:/{}/{}/CUDA/v11.0/lib/x64"#, "Program Files", "NVIDIA GPU Computing Toolkit");
    println!("cargo:rustc-flags=-l dylib=cudart");
    println!("cargo:rustc-flags=-l dylib=cudnn");
    println!("cargo:rustc-flags=-l dylib=cublas");
    println!("cargo:rustc-flags=-l dylib=nvrtc");
}
