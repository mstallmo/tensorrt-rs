use cmake::Config;
use std::process::Command;
use std::string::String;
use std::process::Stdio;
use std::path::PathBuf;

fn get_shared_lib_link_path(library_name: &str) -> Option<PathBuf> {
    match get_all_possible_link_paths(library_name) {
        Some(all_link_paths) => {
            for line in all_link_paths.lines() {
                if line.ends_with(&format!("{}.so", library_name)) {
                    let link_path = line.split("=>").collect::<Vec<&str>>().last().unwrap().to_owned();
                    println!("link path: {}", link_path);
                    return Some(PathBuf::from(link_path.trim().to_owned()));
                }
            }
            None
        }
        None => {
            None
        }
    }
}

fn get_all_possible_link_paths(library_name: &str) -> Option<String> {
    let mut ld_config = Command::new("ldconfig").arg("-p").stdout(Stdio::piped()).spawn().expect("Failed to run ldconfig");

    if let Some(ld_config_output) = ld_config.stdout.take() {
        let grep_config = Command::new("grep").arg(library_name).stdin(ld_config_output).stdout(Stdio::piped()).spawn().unwrap();
        let grep_stdout = grep_config.wait_with_output().unwrap();
        ld_config.wait().unwrap();
        Some(String::from_utf8(grep_stdout.stdout).unwrap())
    } else {
        None
    }
}

#[cfg(feature = "trt-515")]
fn configuration(full_library_path: &PathBuf) {
    if full_library_path.to_str().unwrap().ends_with("5.1.5") {
        let mut config = Config::new("trt-sys");
        let dst = config.define("TRT_VERSION", "5.1.5").build();
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-flags=-l dylib=nvinfer");
        println!("cargo:rustc-flags=-l dylib=nvparsers");
        cuda_configuration();
    } else {
        panic!("Invalid nvinfer version found. Expected: libnvinfer.so.5.1.5, Found: {}", full_library_path.to_str().unwrap());
    }
}

#[cfg(feature = "trt-713")]
fn configuration(full_library_path: &PathBuf) {
    if full_library_path.to_str().unwrap().ends_with("7.1.3") {
        let mut config = Config::new("trt-sys");
        let dst = config.define("TRT_VERSION", "7.1.3").build();
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-flags=-l dylib=nvinfer");
        println!("cargo:rustc-flags=-l dylib=nvparsers");
        cuda_configuration();
    } else {
        panic!("Invalid nvinfer version found. Expected: libnvinfer.so.7.1.3, Found: {}", full_library_path.to_str().unwrap());
    }
}

#[cfg(feature = "cuda-101")]
fn cuda_configuration() {
    println!("cargo:rustc-flags=-L /usr/local/cuda-10.1/lib64");
    println!("cargo:rustc-flags=-l dylib=cudart");
}

#[cfg(feature = "cuda-102")]
fn cuda_configuration() {
    println!("cargo:rustc-flags=-L /usr/local/cuda-10.2/lib64");
    println!("cargo:rustc-flags=-l dylib=cudart");
}

#[cfg(feature = "cuda-110")]
fn cuda_configuration() {
    println!("cargo:rustc-flags=-L /usr/local/cuda-11.0/lib64");
    println!("cargo:rustc-flags=-l dylib=cudart");
}

// Not sure if I love this solution but I think it's relatively robust enough for now on Unix systems.
// Still have to thoroughly test what happens with a TRT library installed that's not done by the
// dpkg. It's possible that we'll just have to fall back to only supporting one system library and assuming that
// the user has the correct library installed and is viewable via ldconfig.
//
// Hopefully something like this will work for Windows installs as well, not having a default library
// install location will make that significantly harder.
fn main() {
    println!("cargo:rustc-link-lib=static=trt-sys");
    println!("cargo:rustc-flags=-l dylib=stdc++");

    match get_shared_lib_link_path("libnvinfer") {
       Some(link_path) => {
          match std::fs::read_link(link_path) {
              Ok(full_library_path) => {
                  configuration(&full_library_path);
              },
              Err(_) => {
                  panic!("libnvinfer.so not found! See https://docs.nvidia.com/deeplearning/tensorrt/archives/tensorrt-515/tensorrt-install-guide/index.html for install instructions");
              }
          }
       },
       None => {
           panic!("libnvinfer.so not found! See https://docs.nvidia.com/deeplearning/tensorrt/archives/tensorrt-515/tensorrt-install-guide/index.html for install instructions");
       }
    }
}
