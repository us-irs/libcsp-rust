use std::{env, path::PathBuf};

pub const ENV_KEY_CSP_CONFIG_DIR: &str = "CSP_CONFIG_DIR";

fn main() {
    println!("cargo:rustc-link-lib=csp");

    let out_path = env::var("OUT_DIR").unwrap();
    let csp_conf_dir = match env::var("CSP_CONFIG_DIR") {
        Ok(conf_path) => conf_path,
        Err(_e) => {
            println!("cargo:warning=CSP_CONFIG_DIR not set, using CARGO_MANIFEST_DIR to search for autoconfig.rs");
            env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set")
        }
    };
    let mut csp_conf_path = PathBuf::new();
    csp_conf_path.push(csp_conf_dir);
    csp_conf_path.push("autoconfig.rs");
    if !csp_conf_path.exists() {
        panic!(
            "autoconfig.rs not found at {:?}, is required for library build",
            csp_conf_path
        );
    }
    let out_path_full = PathBuf::from(&out_path).join("autoconfig.rs");
    std::fs::copy(&csp_conf_path, out_path_full).expect("failed to copy autoconfig.rs to OUT_DIR");
    println!("cargo::rerun-if-changed={:?}", &csp_conf_path);
}
