// build.rs
extern crate bindgen;

use std::{
    env,
    path::PathBuf,
};
//use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("cargo:rerun-if-changed=wrapper.hpp");

    /*
    eprintln!("Building Eigen3");
    let eigen = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("eigen3")
        .unwrap();

    println!("cargo:warning={:?}", eigen.include_paths);*/

    let libfive_path = cmake::build("libfive");

    let mut libfive_include_path = libfive_path.clone();
    libfive_include_path.push("include");

    let mut libfive_lib_path = libfive_path.clone();
    libfive_lib_path.push("lib");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:warning={:?}", libfive_include_path);

    // Emit linker searchpath
    println!("cargo:rustc-link-search={:?}", libfive_lib_path);
    // Link to lib3delight
    println!("cargo:rustc-link-lib=dylib=five");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .derive_debug(true)
        .clang_arg("-std=c++11")
        .clang_arg("-stdlib=libc++")
        .clang_arg("-isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk")
        .clang_arg("-I/usr/local/include/eigen3")
        .clang_arg("-I/usr/local/include")
        .clang_arg(format!("-I{}", libfive_include_path.display()))
        //.clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        //.clang_arg("-F/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk/System/Library/Frameworks/")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate libfive bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    Ok(())
}
