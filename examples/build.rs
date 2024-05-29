use std::{env, path::PathBuf};

use libcsp_cargo_build::Builder;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Pass some important build script environment variables to the binary/library.
    // Remove this at a later stage, this belongs in a concrete example app.
    /*
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!(
        "cargo:rustc-env=OPT_LEVEL={}",
        std::env::var("OPT_LEVEL").unwrap()
    );
    println!("cargo:rustc-env=HOST={}", std::env::var("HOST").unwrap());
    */

    // Tell cargo to tell rustc to link our `csp` library. Cargo will
    // automatically know it must look for a `libcsp.a` file.
    // println!("cargo:rustc-link-lib=csp");
    // println!("cargo:rustc-link-search={}/csp", project_dir);

    let mut csp_builder = Builder::new();
    csp_builder.compile();
}
