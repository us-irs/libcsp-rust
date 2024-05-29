#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
extern "C" {
    pub fn csp_print_func(fmt: *const ::std::os::raw::c_char, ...);
    #[doc = " Initialize CSP.\n This will configure basic structures."]
    pub fn csp_init();
}

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// extern "C" {
    // pub fn csp_print_func(fmt: *const ::std::os::raw::c_char, ...);
//}
