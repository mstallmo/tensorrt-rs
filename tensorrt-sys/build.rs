use cmake::Config;

fn main() {
    let dst = Config::new("trt-sys").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=trt-sys");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l dylib=nvinfer");
    println!("cargo:rustc-flags=-l dylib=nvparsers");
    println!("cargo:rustc-flags=-L /usr/local/cuda/lib64");
    println!("cargo:rustc-flags=-l dylib=cudart");
}
