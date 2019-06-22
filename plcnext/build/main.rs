use std::env;

fn main() {
    let mut my_config = cpp_build::Config::new();
    // TODO: Replace the path with an ENV var.
    // Check this for guidance:
    // https://github.com/woboq/qmetaobject-rs/blob/master/qmetaobject/build.rs
    my_config.include("/opt/pxc/sdk/AXCF2152/2019.3/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext");
    my_config.build("src/lib.rs");

    // Tell cargo to tell rustc to link the correct libraries.
    println!("cargo:rustc-link-lib=Arp.Device.Interface");
    println!("cargo:rustc-link-lib=Arp.Io.Axioline");
}
