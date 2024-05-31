libcsp-rust
=========

This project aims to provide libraries and tools to use
[`libcsp`](https://github.com/libcsp/libcsp) in your Rust project.

It provides 2 crates for this:

- [`libcsp-cargo-build`](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/libcsp-cargo-build)
  provides an API to build the `libcsp` using `cargo` with the [`cc`](https://docs.rs/cc/latest/cc/) crate.
- [`libcsp-rust`](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/libcsp-rust)
  provides the Rust bindings to `libcsp` and a safe and ergonomic Rust interface.

In addition, it provides a workspace to allow updating the `libcsp` and the corresponding bindings 
more easily inside the `lib` directory. Some of the examples `libcsp` provides were ported to Rust
and are showcases in the `examples` directory.

## How it works

We assume that cargo should also take care of building the library.

1. Add the `libcsp-cargo-build` as a build dependency inside your `Cargo.toml`.
2. Add the `libcsp-rust` as a regular dependency inside your `Cargo.toml`.
3. Create a custom `build.rs` script which takes care of building `libcsp` using the API
   provided by `libcsp-cargo-build`. You have to provide the source code for `libcsp` inside some
   directory and pass the directory path to a builder API.
4. You can now write regular Rust code and use the API provided by `libcsp-rust` to use `libcsp`
   in a safe and Rusty way.

It is recommended to have a look at the [example build script](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/build.rs)
which should give you a general idea of how a build script might look like to integrate `libcsp`.

## Running the example

The example uses both the builder crate and the bindings and API crate and implements the
[server/client example](https://github.com/libcsp/libcsp/blob/develop/examples/csp_server_client.c)
in Rust. You can run the example using the following steps:

1. Clone `libcsp` into the `lib` folder, for example by using the provided `lib/clone-csp.sh`
   script.
2. You can now use `cargo run -p libcsp-rust-examples` to run the server/client example.

## Compile-time configuration of the `libcsp-rust` library

The `libcsp-rust` library requires some compile-time configuration file to be included to work
properly. You can see an example version of the file for the workspace
[here](https://egit.irs.uni-stuttgart.de/rust/libcsp-rust/src/branch/main/examples/autoconfig.rs).
The user has to provide the path to a directory containing this `autoconfig.rs` file using the
`CSP_CONFIG_DIR` environmental variable. 

You can automatically generate this file when using `libcsp-cargo-build` by using the
[`generate_autoconf_rust_file`] method of the Builder object as done in the example build script.
In this workspace, the `CSP_CONFIG_DIR` variable is hardcoded using the following `.cargo/config.toml`
configuration:

```toml
[env]
CSP_CONFIG_DIR = { value = "examples", relative = true }
```

## Generating and update the bindings using the `lib` folder

The `lib` folder in this repository serves as the staging directory for the `libcsp` library to
build. However, it can also be used to update the bindings provided in `libcsp-rust` by providing
some tools and helpers to auto-generate and update the bindings file `bindings.rs`.

If you want to do this, you should install `bindgen-cli` first:

```sh
cargo install bindgen-cli --locked
```

`bindgen` needs some additional information provided by the user to generate the bindings:
An `autoconfig.h` file which is used to configure `libcsp`. Normally, this file is generated
by the C build system. This file is located at `cfg/csp` and is also updated automatically
when running the example application.

After cloning the repository, you can now run the following command to re-generate the bindings
file:

```sh
bindgen --use-core wrapper.h -- "-I./libcsp/include" "-I./cfg" "-I./libcsp/src" > bindings.rs
```

With the bindings file, you can now manually update the FFI bindings provided in
`libcsp-rust/src/ffi.rs` or in your own CSP library.

