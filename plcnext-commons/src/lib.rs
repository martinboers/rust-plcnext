#[macro_use]
extern crate cpp;

// use std::ffi::CStr;
use std::ffi::CString;

cpp!{{
    #include "Arp/System/Commons/Logging.h"
}}

pub fn main() {
    unsafe {
        cpp!([] {
            int a = 12;
            Arp::Log::Info("hello");
        });
    }
}

pub fn log(msg: &str) {
    let text = CString::new(msg).expect("CString::new(msg) failed");;
    let raw_text = text.into_raw();
    unsafe {
        cpp!([raw_text as "const char *"] {
            Arp::Log::Info(raw_text);
        });
    }
}
