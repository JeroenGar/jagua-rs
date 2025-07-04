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

This avoids runtime errors with crates like `rand` that rely on `getrandom` under the hood.

> See: [getrandom WebAssembly support](https://docs.rs/getrandom/0.3.3/#webassembly-support)

## 3. WASM-Specific Compatibility Changes

### 3.1 Replacing `std::time::Instant`

Because `std::time::Instant` is not portable to WASM (especially in browser environments), it was replaced throughout the codebase with a millisecond-resolution `f64` timestamp, derived from `performance.now()` via `web-sys`.

### 3.2 BPProblem and SPProblem API Changes

A small but essential change was made to the `BPProblem::save()` and `SPProblem::save()` API:

```rust
// Previous:
self.problem.save()

// New:
let time = now_millis(); // platform-specific time in f64
self.problem.save(time);

// Can also use:

let time = TimeStamp::now();
self.problem.save(time.elapsed_ms());
```

This preserves time metadata without relying on non-WASM-compatible structures.

### 3.3 Other Changes

* Logic was added to safely convert `f64` timestamps to `u64` when needed.
* Minor adjustments made to ensure deterministic behavior across environments.
* `BPSolution` and `SPSolution` have Wasm32 specific structs now for the `time_stamp` param.

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
