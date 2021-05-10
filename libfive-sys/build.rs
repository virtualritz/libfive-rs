// build.rs
//extern crate bindgen;

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Dependencies -----------------------------------------------------------

    /*
    #[cfg(target_os = "macos")]
    let lib = vcpkg::find_package("eigen");

    #[cfg(target_os = "linux")]
    #[cfg(target_os = "windows")]
    let lib = vcpkg::Config::new().copy_dlls(true).probe("eigen");

    */

    // libfive ----------------------------------------------------------------

    let mut libfive_builder = cmake::Config::new("libfive");

    libfive_builder.define("BUILD_STUDIO_APP", "OFF");
    libfive_builder.define("BUILD_GUILE_BINDINGS", "OFF");
    libfive_builder.define("BUILD_PYTHON_BINDINGS", "OFF");

    #[cfg(feature = "packed_opcodes")]
    libfive_builder.define("LIBFIVE_PACKED_OPCODES", "ON");

    let libfive_path = libfive_builder.build();

    let mut libfive_include_path = libfive_path.clone();
    libfive_include_path.push("include");

    let mut libfive_lib_path = libfive_path;
    libfive_lib_path.push("lib");

    let mut stdlib_include_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    stdlib_include_path.push("libfive");
    stdlib_include_path.push("libfive");
    stdlib_include_path.push("stdlib");

    // Emit linker searchpath
    println!("cargo:rustc-link-search={}", libfive_lib_path.display());
    // Link to libfive
    println!("cargo:rustc-link-lib=five");
    println!("cargo:rustc-link-lib=five-stdlib");

    // libfive wrapper --------------------------------------------------------

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_debug(true)
        .derive_eq(true)
        .allowlist_type("libfive.*")
        .allowlist_function("libfive.*")
        //.opaque_type("_.*")
        //.blocklist_item("_.*")
        //.blocklist_constants("*")
        .clang_arg(
            "-isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk",
        )
        //.clang_arg("-I/usr/local/include/eigen3")
        //.clang_arg("-I/usr/local/include")
        .clang_arg(format!("-I{}", libfive_include_path.display()))
        .clang_arg(format!("-I{}", stdlib_include_path.display()))
        .generate()
        .expect("Unable to generate libfive bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    Ok(())
}
