libcsp-sys
========

This crate provides FFI bindings for the [`libcsp` library](https://github.com/libcsp/libcsp).

Generally, you probably do not want to use this library directly and instead use the
`libcsp` Rust crate which provides a safe and ergonomic Rust API.
You can find some more high-level information and examples in the
[main repository](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust).

## Compile-time configuration of the `libcsp-rust` library

The `libcsp-rust` library requires some compile-time configuration file to be included to work
properly. You can see an example version of the file for the workspace
[here](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/autoconfig.rs).
The user has to provide the path to a directory containing this `autoconfig.rs` file using the
`CSP_CONFIG_DIR` environmental variable.

It is recommended to read the [main workspace README](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust)
for more information to make the generation and specification of this auto-configuration file
as conveniently and easy as possible.
