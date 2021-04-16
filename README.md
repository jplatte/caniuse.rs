# caniuse.rs

A website to know which feature you can or cannot use in Rust.

## Quickstart

Head over [caniuse.rs](https://caniuse.rs/) and search for the Rust feature you are interested in.

## Developpement

The project uses a standard Rust toolchain.

### Pre-requisites

- **Cargo**
- **wasm-pack**: `cargo install wasm-pack`
- **Python3**
- **rollup**: `npm i -g rollup`

### Commands

Once the requirements are filled up you can run the project with `./x.py serve`

### Creates a new feature

You may want to look at issue [#16](https://github.com/jplatte/caniuse.rs/issues/16) for missing features.

When adding a new feature check on `FeatureData` in `build.rs` file for every available options