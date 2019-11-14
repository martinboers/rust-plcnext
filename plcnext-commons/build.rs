extern crate cpp_build;

fn main() {
    let mut my_config = cpp_build::Config::new();
    my_config.include("/opt/pxc/sdk/AXCF2152/2019.9/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext");
    my_config.build("src/lib.rs");
    println!("cargo:rustc-link-lib=cppformat");
}