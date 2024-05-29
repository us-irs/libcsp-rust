use libcsp_rust::csp_init;

fn main() {
    println!("Hello, world!");
    unsafe {
        csp_init();
    }
}
