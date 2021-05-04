// build.rs
extern crate bindgen;

use std::{
    env,
    path::PathBuf,
};
//use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=wrapper.hpp");

    let libfive_path = cmake::Config::new("libfive")
        .define("BUILD_STUDIO_APP", "OFF")
        .define("BUILD_GUILE_BINDINGS", "OFF")
        .define("BUILD_PYTHON_BINDINGS", "OFF")
        .build();

    let mut libfive_include_path = libfive_path.clone();
    libfive_include_path.push("include");

    let mut libfive_lib_path = libfive_path.clone();
    libfive_lib_path.push("lib");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Emit linker searchpath
    println!("cargo:rustc-link-search={:?}", libfive_lib_path);
    // Link to lib3delight
    println!("cargo:rustc-link-lib=dylib=five");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_debug(true)
        .allowlist_type("libfive.*")
        .allowlist_function("libfive.*")
        .clang_arg("-isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk")
        .clang_arg("-I/usr/local/include/eigen3")
        .clang_arg("-I/usr/local/include")
        .clang_arg(format!("-I{}", libfive_include_path.display()))
        .generate()
        .expect("Unable to generate libfive bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    Ok(())
}
