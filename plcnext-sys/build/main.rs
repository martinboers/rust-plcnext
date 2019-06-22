extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the PLCnext ANSI-C libraries.
    println!("cargo:rustc-link-lib=Arp.Plc.AnsiC");
    println!("cargo:rustc-link-lib=Arp.System.ModuleLib");
    println!("cargo:rustc-link-lib=Arp.System.Module");
    println!("cargo:rustc-link-lib=Arp.System.Rsc");
    println!("cargo:rustc-link-lib=Arp.System.Commons");
    // println!("cargo:rustc-link-lib=stdc++");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // Note that clang arguments for the build must be
    // supplied using the BINDGEN_EXTRA_CLANG_ARGS env variable.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}