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

> [!NOTE]
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

Some changes were made to the core `jagua-rs` crate in this fork.

These changes were mainly made to ensure that the WASM-Build is logically intact and shows *very similar* results.

1. Changed all instances of `std::time::Instant` to `f64` for WASM friendly primitive that works with `performance()` (in browser and node)
2. Added a parameter to BPProblem's `save` function called `time_stamp` of `f64` type 

The API change would be:

```rust 
// prev
self.problem.save()
// now 
let time = now_millis();
self.problem.save(time);
```

3. Some extra logic when converting from `f64` to `u64` when exporting 
