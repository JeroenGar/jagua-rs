# Developer Notes for `jagua-rs` – WASM Parallelism Support

> [!IMPORTANT]
> 
> No code was removed, so any mention of the word "replaced", 
> simply means that the initial logic was tweaked as a workaround
> and that workaround was added under a conditional macro for Wasm
> 

This document outlines the caveats, challenges, and implementation-specific decisions taken to support WebAssembly (WASM) and parallelism in the `jagua-rs` crate. The focus is on minimal disruption to the existing native Rust build while enabling efficient multithreaded execution in the browser via `wasm-bindgen-rayon`.

## Table of Contents

1. [Introduction](#introduction)
2. [New Dependencies](#new-dependencies)
3. [WASM-Specific Compatibility Changes](#wasm-specific-compatibility-changes)

   * 3.1 [Replacing `std::time::Instant`](#replacing-stdtimeinstant)
   * 3.2 [BPProblem and SPProblem API Changes](#bpproblem-api-changes)
   * 3.3 [Other Changes](#other-changes)
4. [The `separation-distance` Feature](#the-separation-distance-feature)

   * 4.1 [Why It Was Disabled](#why-it-was-disabled)
   * 4.2 [Dependencies and Their Issues](#dependencies-and-their-issues)
   * 4.3 [Compiling C++ to WASM: Why It's Problematic](#compiling-c-to-wasm-why-its-problematic)
   * 4.4 [Why Emscripten Was Rejected](#why-emscripten-was-rejected)
   * 4.5 [Resolution](#resolution)

## 1. Introduction

The goal of this work was to enable `jagua-rs` to run in the browser with multithreaded WebAssembly support, using the `wasm-bindgen-rayon` crate. This required changes in the codebase, careful dependency selection, and adaptations to platform limitations of the browser-based WASM runtime.

## 2. New Dependencies

The following dependencies were added to support the WASM runtime and multithreaded execution:

In the root `Cargo.toml`:

```toml
wasm-bindgen = { version = "0.2", features = ["serde"] }
web-sys = { version = "0.3", features = ["Window", "Performance"] }
serde-wasm-bindgen = "0.6"
wasm-bindgen-rayon = "1.3"
console_error_panic_hook = "0.1"
```

Additionally, the `getrandom` crate was added explicitly with WASM support enabled:

```toml
getrandom = { version = "0.3", default-features = false, features = ["wasm_js"] }
```

We should also ensure that both `jagua-rs` and `lbf`'s `Cargo.toml` files have this:

```toml 
getrandom = { workspace = true }

# Only compile these when targeting wasm32
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = { workspace = true }
wasm-bindgen-rayon = { workspace = true }
serde-wasm-bindgen = { workspace = true }
console_error_panic_hook = { workspace = true }
console_log = { version = "1.0", features = ["color"] }
web-sys = { workspace = true }
```

This avoids runtime errors with crates like `rand` that rely on `getrandom` under the hood.

> See: [getrandom WebAssembly support](https://docs.rs/getrandom/0.3.3/#webassembly-support)

Finally, we need to create a `.cargo/config.toml` in the `lbf` directory:

```toml 
[target.wasm32-unknown-unknown]
rustflags = [
  "--cfg", "getrandom_backend=\"wasm_js\"",
  "-C", "target-feature=+atomics,+bulk-memory,+simd128",
]

[unstable]
build-std = ["panic_abort", "std"]
```

This config lets cargo know the following:

a. Set the `getrandom` backend for support for `wasm_js` when compiling to `wasm32-unknown-unknown` target 
b. Set target features of atomics + bulk-memory + simd128 on when compiling to `wasm32-unknown-unknown` target (needed for multithreading)
c. To rebuild the standard library for point b (for threading in Wasm)

## 3. WASM-Specific Compatibility Changes

#### 3.1 Accounting for `std::time::Instant`

Because `std::time::Instant` is not portable to WASM (especially in browser environments), we needed a simple modular way to handle timestamps for both native and Wasm counterparts.

This is why I added a extra file [time.rs](https://github.com/nots1dd/jagua-rs/blob/wasm-parallel/lbf/src/time.rs) in `lbf/src`

This Rust file defines a cross-platform time abstraction called TimeStamp, designed to work both in:

- Native environments (like Linux/macOS/Windows)
- Wasm environments (e.g., in the browser via wasm32)

You can check out the implementation for it, it is quite simple; it provides a `TimeStamp` enum with a lot of methods that have internal logic depending on the `target_arch`. Here is a small snippet:

```rust 
/// Get a new timestamp for "now"
pub fn now() -> Self {
    #[cfg(not(target_arch = "wasm32"))]
    {
        /// This is std::time::Instant --> NOT Wasm COMPATIBLE!!
        TimeStamp::Instant(Instant::now())
    }

    #[cfg(target_arch = "wasm32")]
    {
        /// Custom function that uses web_sys's performace() --> Wasm COMPATIBLE!!
        TimeStamp::Millis(now_millis())
    }
}
```

Now, we replace all instances of using `std::time::Instant` with: 

```rust 
Instant::now(); // OLD

/// This internally deals with whether the target_arch is wasm or not!
TimeStamp::now() // NEW
```

### 3.2 BPProblem and SPProblem API Changes

A small but essential change was made to the `BPProblem::save()` and `SPProblem::save()` API (for Wasm only):

```rust
// Previous:
self.problem.save() // only works in native

// New:
let time = now_millis(); // platform-specific time in f64
#[cfg(target_arch = "wasm32")]
self.problem.save(time); // Wasm friendly save function
#[cfg(not(target_arch = "wasm32"))]
self.problem.save() // native save function

// Can also use:

let time = TimeStamp::now();
#[cfg(target_arch = "wasm32")]
self.problem.save(time.elapsed_ms());
```

This preserves time metadata without relying on non-WASM-compatible structures.

### 3.3 Other Changes

* Logic was added to safely convert `f64` timestamps to `u64` when needed.
* `BPSolution` and `SPSolution` have Wasm32 specific structs now for the `time_stamp` param.
* Added a logger for Wasm + native in [lib.rs](https://github.com/nots1dd/jagua-rs/blob/wasm-parallel/lbf/src/lib.rs)
* Modified the current [init_logger](https://github.com/nots1dd/jagua-rs/blob/wasm-parallel/lbf/src/io/mod.rs)
* Rng fallback was modified (in config's prng_seed) for Wasm:

```rust 
SmallRng::from_os_rng(); // Native solution: not Wasm compatible

// A more determisnistic fallback for rng seed (only invoked if config does not have prng_seed
SmallRng::seed_from_u64(0x12345678); // Wasm friendly solution
```

> [!WARNING]
> 
> The 4th point is deprecated and this crate is no longer used in the 
> latest commit of jagua-rs, but I have still kept this to showcase the workaround 
> I came up at the time.
> 

## 4. The `separation-distance` Feature

This feature was disabled due to its reliance on non-Rust dependencies that are incompatible with `wasm32-unknown-unknown`.

### 4.1 Why It Was Disabled

The feature uses geometric operations to ensure polygons maintain a certain minimum separation. However, its dependency stack introduces compatibility issues.

### 4.2 Dependencies and Their Issues

From `Cargo.toml`:

```toml
geo-offset = { version = "0.4.0", optional = true }
geo-types = { version = "0.7.16", optional = true }
```

`geo-offset` internally depends on `geo-clipper`, which in turn depends on `clipper-sys`, a Rust FFI wrapper for the C++ `Clipper` library.

### 4.3 Compiling C++ to WASM: Why It's Problematic

C++ code can only be compiled to WASM using **Emscripten**, which provides a C++ standard library in a WASM-compatible format. However:

* This cannot be used with the `wasm32-unknown-unknown` target.
* `wasm-bindgen` and `wasm-bindgen-rayon` are not compatible with Emscripten.
* Threading, memory sharing, and JS bindings would all need to be rewritten.

### 4.4 Why Emscripten Was Rejected

Using `wasm32-unknown-emscripten` would:

* Break `wasm-bindgen`-based integration.
* Prevent usage of `wasm-bindgen-rayon`.
* Require maintaining a dual toolchain and completely different build and integration workflow.

These trade-offs were deemed too costly for minimal functional gain.

### 4.5 Resolution

The preferred solution is to switch to a **pure Rust** alternative for polygon offsetting.

#### Suggested Alternative:

* [`geo + geo-buffer`](https://crates.io/crates/geo) – performs geometric boolean operations and is compatible with `wasm32-unknown-unknown`.

This issue has since been resolved by jagua-rs maintainer in this [commit](https://github.com/JeroenGar/jagua-rs/pull/38). I have put this here regardless to showcase how I approached the problem and the modifications I had to make at the time (you *may* come across issues like this too)
