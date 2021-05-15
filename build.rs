//! Parse and collect inserts into inserts file. 
//! After, uses library cmake for building result file as library.
//!
//! NOTE: Uses a modified version cpp_build in which the build was removed.

extern crate cpp_build;
extern crate cmake;
use std::process::Command;

use cpp_build::{create_lib, CPP_DIR};
use cmake::Config;
use std::array;

fn link_libs(libs: &[&str]) {
    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

fn main() {
    let inserts_path = create_lib("src/main.rs");
    let lib_path = "/usr/lib/myexternallib";
    // Config::new(CPP_DIR.as_path())
    //     .env("LLVM_DIR", env!("LLVM_DIR"))
    //     .env("MLIR_DIR", env!("MLIR_DIR"))
    //     .build(); 
    let status = Command::new("cmake")
        .args(&[
            "-S", "./",
            "-B", &CPP_DIR.join("build").as_path().to_string_lossy(),
        ])
        // .current_dir(CPP_DIR.as_path())
        .envs(array::IntoIter::new([
            ("LLVM_DIR", env!("LLVM_DIR")),
            ("MLIR_DIR", env!("MLIR_DIR"))
        ]))
        .status()
	    .expect("failed to execute cmake");
	
	if !status.success() {
		panic!("cmake execution error: {}", status);
	} 

    let status = Command::new("cmake")
        .current_dir(CPP_DIR.join("build").as_path())
        .args(&["--build", "."])
        .status()
	    .expect("failed to execute cmake");

    if !status.success() {
        panic!("cmake execution error: {}", status);
    } 

    let OUT_DIR = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {}", OUT_DIR.to_string_lossy());  
    println!("cargo:rustc-link-search=native={}", OUT_DIR.to_string_lossy());  
    
    println!("cargo:rustc-link-search=native={}", "/home/mrsmith/Desktop/code_experiments/llvm-project/build/lib/"); 
    
    // link_libs(&[

    // ]);

    println!("cargo:rustc-link-lib=static=rust_cpp_generated");
    println!("cargo:rustc-link-lib=static=stdc++");
}