libcsp-rust
========

This crate provides Rust FFI bindings and a Rusty API for the [`libcsp` library](https://github.com/libcsp/libcsp).
You can find some more high-level information and examples in the
[main workspace](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust).

The API documentation should provide all additional information required to use this library.

## Compile-time configuration of the `libcsp-rust` library

The `libcsp-rust` library requires some compile-time configuration file to be included to work
properly. You can see an example version of the file for the workspace
[here](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/autoconfig.rs).
The user has to provide the path to a directory containing this `autoconfig.rs` file using the
`CSP_CONFIG_DIR` environmental variable.

It is recommended to read the [main workspace README](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust)
for more information to make the generation and specification of this auto-configuration file
as conveniently and easy as possible.
