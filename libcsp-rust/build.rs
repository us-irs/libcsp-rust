use std::{env, path::PathBuf};

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
    println!("cargo:rustc-link-lib=csp");
    // println!("cargo:rustc-link-search={}/csp", project_dir);

    // let mut csp_builder = Builder::new();
    // csp_builder.compile();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    /*
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
    */
    if let Ok(_str) = env::var("GEN_BINDINGS") {
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .clang_arg("-Ilibcsp/include")
            .clang_arg("-Icfg")
            .header("wrapper.h")
            .use_core()
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        let local_path = PathBuf::from("./bindings.rs");
        bindings
            .write_to_file(local_path)
            .expect("Couldn't write bindings!");
    }
}
