
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern "C" {
    pub fn csp_print_func(fmt: *const ::std::os::raw::c_char, ...);
}

fn main() {
    println!("Hello, world!");
    unsafe { csp_print_func("Hello, world!\n".as_ptr() as *const i8); }
}
