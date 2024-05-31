libcsp-rust
=========

This project aims to provide libraries and tools to use
[`libcsp`](https://github.com/libcsp/libcsp) in your Rust project.

It provides 2 crates for this:

- [`libcsp-cargo-build`] provides an API to build the `libcsp` using `cargo` with the
  [`cc`](https://docs.rs/cc/latest/cc/) crate.
- [`libcsp-rust`] provides the Rust bindings to `libcsp` and a safe and ergonomic Rust interface.

In addition, it provides a workspace to allow updating the `libcsp` and the corresponding bindings 
more easily inside the `lib` directory. Some of the examples `libcsp` provides were ported to Rust
and are showcases in the `examples` directory.

## Getting started

We assume that cargo should also take care of building the library.

1. Add the `libcsp-cargo-build` as a build dependency inside your `Cargo.toml`.
2. Add the `libcsp-rust` as a regular dependency inside your `Cargo.toml`.
3. Create a custom `build.rs` script which takes care of building `libcsp` using the API
   provided by `libcsp-cargo-build`. You have to provide the source code for `libcsp` inside some
   directory and pass that director to a builder API.
4. You can now write regular Rust code and use the API provided by `libcsp-rust` to use `libcsp`
   in a safe and Rusty way.

It is recommended to have a look at the [example build script]() which should give you a general
idea of how a build script might look like to integrate `libcsp`.
