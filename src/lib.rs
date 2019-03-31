static NOTICE: &'static str = "Rust runtime running.";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[no_mangle]
pub extern "C" fn start() {
    let path = Path::new("Runtime.log");
    let display = path.display();

    // Open a file in write-only mode, returns 'io::Result<File>'
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write text to 'file', returns 'io::result<()>'
    match file.write_all(NOTICE.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                             why.description())
        },
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}