# jagua-rs for Wasm

jagua-rs but with support for Wasm! (with parallelization support through `wasm-bindgen-rayon`)

## Dependencies 

1. **rustup**, **rustc** and **cargo** -- (Rust toolchain kit)
2. **wasm-pack** -- (WASM Bindgen builder for Rust)
3. **wasm-opt** -- Binaryen CLI tool for optimizing Wasm bytecode (**Optional**)
4. **sed**, **bash** and other *NIX utils -- (for the automated building)
5. **python** and networking utils -- (to run the server with COOP && COEP headers enabled)

> [!IMPORTANT]
> 
> This project builds the project on the `nightly-x86_64-unknown-linux-gnu` build.
> 
> So it might be important for you to do this prior to the build:
> 
> ```bash 
> rustup toolchain install nightly
> rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
> ```
> 
> Nightly is important to rebuild the `std` libs for WASM that is critical for `wasm-bindgen-rayon`.
> 

## Building

An example:

```bash 
./build-wasm.sh --target web
```

If you want more information on how the build script works and what it can offer:

```bash 
./build-wasm.sh -h
```

> [!NOTE]
> 
> This build **ONLY** works for **web** target!!
> 
> This is due to the fact that we are using `SharedArrayBuffer` that only works 
> on browser runtime.
> 


## Running

```bash 
python serve.py
```

> [!IMPORTANT]
> 
> Running and testing through this script is important as it enables **COOP** and **COEP** headers
> (cross-origin isolated environment headers) that are necessary in order to access `SharedArrayBuffer`
> for multi-threading in WASM runtimes.
> 

You can have an express server or other methods of running this as well but this is quite simple and easy to use.

Now open `http://localhost:8081/index.html` and see it work.

## Changes

> [!IMPORTANT]
> 
> Some changes were made to the core `jagua-rs` and `lbf` crates in this fork.
> 
> Check out [DEV.md](https://github.com/nots1dd/jagua-rs/tree/wasm-parallel/DEV.md) for more info.
> 
