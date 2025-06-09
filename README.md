# jagua-rs for WASM

> [!WARNING]
> 
> The `lbf` directory has been made to completely support **ONLY** `wasm32-unknown-unknown` target 
> and **cannot** be compiled like mentioned in the main repository. (using cargo)
> 

This particular fork of jagua-rs implements the algorithm for a WASM target for the web (`wasm32-unknown-unknown` in particular)

## Dependencies 

1. **rustup**, **rustc** and **cargo** -- (Rust toolchain)
2. **wasm-pack** -- (WASM Bindgen builder for Rust)
3. **sed**, **bash** and other *NIX utils -- (for the automated building)
4. **python** and networking utils -- (to run the server with COOP && COEP headers enabled)

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

```bash 
# in the root dir
./build-wasm.sh --target <provide-target>
```

An example:

```bash 
./build-wasm.sh --target web
```

> [!NOTE]
> 
> This build **ONLY** works for **web** target!!
> 
> This is due to the fact that we are using `SharedArrayBuffer` that only works 
> on v8 (browser) runtime.
> 


## Running

```bash 
cd lbf 
python serve.py
```

> [!IMPORTANT]
> 
> Running and testing through this script is important as it enables **COOP** and **COEP** headers
> (cross-origin isolated environment headers) that are necessary in order to access `SharedArrayBuffer`
> for multi-threading in WASM runtimes.
> 

You can have an express server or other methods of running this as well but this is quite simple and easy to use.

Now open `http://localhost:8080/index.html` and see it work.

## Changes

> [!IMPORTANT]
> 
> Some changes were made to the core `jagua-rs` crate in this fork.
> 
> Check out [DEV.md](https://github.com/nots1dd/jagua-rs/tree/wasm-parallel/DEV.md) for more info.
> 
