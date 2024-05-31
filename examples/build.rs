use std::{env, path::PathBuf};

use libcsp_cargo_build::Builder;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or_default();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let libcsp_path = "../lib/libcsp";
    let mut csp_builder = Builder::new(PathBuf::from(libcsp_path), PathBuf::from(&out_dir));
    let update_autoconf = match env::var("UPDATE_CSP_AUTOCONF") {
        Ok(update_autoconf) => update_autoconf == "1",
        Err(_e) => false,
    };
    if update_autoconf {
        csp_builder
            .generate_autoconf_rust_file(PathBuf::from(&manifest_dir))
            .expect("generating autoconfig.rs failed");
        println!("cargo:warning=autoconfig.rs updated");
    }
    csp_builder.compile().expect("compiling libcsp failed");
}
