// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern "C" {
    pub fn csp_print_func(fmt: *const ::std::os::raw::c_char, ...);
    #[doc = " Initialize CSP.\n This will configure basic structures."]
    pub fn csp_init();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        csp_init();
    }
}
