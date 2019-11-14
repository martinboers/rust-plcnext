extern crate cpp_build;
use std::env;

fn main() {
    let mut my_config = cpp_build::Config::new();
    match env::var("PLCNEXT_HEADERS") {
        Ok(value) => my_config.include(value).build("src/lib.rs"),
        // Err(e) => println!("Couldn't retrieve value of PLCNEXT_HEADERS: {}", e),
        Err(_e) => my_config.build("src/lib.rs"),
    }
    // my_config.include("/opt/pxc/sdk/AXCF2152/2019.9/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext");
    // my_config.build("src/lib.rs");
    println!("cargo:rustc-link-lib=cppformat");
}