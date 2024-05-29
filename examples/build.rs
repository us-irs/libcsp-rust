use std::{env, path::PathBuf};

use libcsp_cargo_build::Builder;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or_default();

    let libcsp_path = "../lib/libcsp";
    let mut csp_builder = Builder::new(PathBuf::from(libcsp_path), PathBuf::from(out_dir));
    csp_builder.compile().expect("compiling libcsp failed");
}
