use cmake::Config;
use std::env;

//TODO: Finish this linker path and implement for 7.1.0
// Figure out why the ldd for the example executable is still finding the nvinfer at the default
// location instead of our installed location. Potentially look into making the trt-sys library a
// dynamic library instead of static to handle the linking there rather than dealing with it on the
// Rust side.
fn main() {
    let mut config = Config::new("trt-sys");
    println!("cargo:rustc-link-lib=static=trt-sys");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    #[cfg(feature = "trt-515")]
    {
        let dst = config.define("TRT_VERSION", "5.1.5").build();
        let curr_dir = env::current_dir().unwrap();
        println!("current directory: {}", curr_dir.display());
        println!("library directory: {}/trt-sys/libs/TensorRT-5.1.5.0/lib", curr_dir.display());
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-flags=-L {}/trt-sys/libs/TensorRT-5.1.5.0/lib", curr_dir.display());
        println!("cargo:rustc-flags=-l dylib=nvinfer");
        println!("cargo:rustc-flags=-l dylib=nvparsers");
        println!("cargo:rustc-flags=-L /usr/local/cuda-10.1/lib64");
        println!("cargo:rustc-flags=-l dylib=cudart");
    }
}
