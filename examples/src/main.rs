use std::ffi::CString;

use libcsp_rust::{csp_init, csp_print_func};

fn main() {
    println!("Hello, world!");
    unsafe {
        csp_init();
        let c_str = CString::new("hello world\n").unwrap();
        csp_print_func(c_str.as_ptr());
    }
}
