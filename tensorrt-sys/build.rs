use cmake::Config;

fn main() {
    let dst = Config::new("trt-sys")
                     .generator("Ninja")
                     .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=trt-sys");
//    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-L D:/projects/TensorRT-7.1.3.4/lib");
    println!("cargo:rustc-flags=-l dylib=nvinfer");
    println!("cargo:rustc-flags=-l dylib=nvparsers");
//    println!("cargo:rustc-flags=-L /usr/local/cuda/lib64");
    println!(r#"cargo:rustc-link-search=-L C:/{}/{}/CUDA/v11.0/lib/x64"#, "Program Files", "NVIDIA GPU Computing Toolkit");
//    println!("cargo:rustc-flags=-l dylib=cudart");
}
