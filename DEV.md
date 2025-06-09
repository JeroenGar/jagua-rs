# Developer Notes for `jagua-rs` – WASM Parallelism Support

This document outlines the caveats, challenges, and implementation-specific decisions taken to support WebAssembly (WASM) and parallelism in the `jagua-rs` crate. The focus is on minimal disruption to the existing native Rust build while enabling efficient multithreaded execution in the browser via `wasm-bindgen-rayon`.

## Table of Contents

1. [Introduction](#introduction)
2. [New Dependencies](#new-dependencies)
3. [WASM-Specific Compatibility Changes](#wasm-specific-compatibility-changes)

   * 3.1 [Replacing `std::time::Instant`](#replacing-stdtimeinstant)
   * 3.2 [BPProblem API Changes](#bpproblem-api-changes)
   * 3.3 [Other Changes](#other-changes)
4. [Logging and Configuration](#logging-and-configuration)
5. [Removed Components](#removed-components)
6. [The `separation-distance` Feature](#the-separation-distance-feature)

   * 6.1 [Why It Was Disabled](#why-it-was-disabled)
   * 6.2 [Dependencies and Their Issues](#dependencies-and-their-issues)
   * 6.3 [Compiling C++ to WASM: Why It's Problematic](#compiling-c-to-wasm-why-its-problematic)
   * 6.4 [Why Emscripten Was Rejected](#why-emscripten-was-rejected)
   * 6.5 [Resolution](#resolution)
7. [Future Considerations](#future-considerations)

## 1. Introduction

The goal of this work was to enable `jagua-rs` to run in the browser with multithreaded WebAssembly support, using the `wasm-bindgen-rayon` crate. This required surgical changes in the codebase, careful dependency selection, and adaptations to platform limitations of the browser-based WASM runtime.

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

### 3.2 BPProblem API Changes

A small but essential change was made to the `BPProblem::save()` API:

```rust
// Previous:
self.problem.save()

// New:
let time = now_millis(); // platform-specific time in f64
self.problem.save(time);
```

This preserves time metadata without relying on non-WASM-compatible structures.

### 3.3 Other Changes

* Logic was added to safely convert `f64` timestamps to `u64` when needed.
* Minor adjustments made to ensure deterministic behavior across environments.

## 4. Logging and Configuration

* Logging remains in the codebase but is **not initialized** in the `lbf` crate when compiled to WASM.
* All configurations are **defaulted** for the WASM build to minimize setup complexity.

## 5. Removed Components

The **SPP (Sequential Polygon Packing)** logic was removed as it is unused in the current WASM-oriented implementation. It can be restored if needed but is not essential for the parallel version of the layout solver.

## 6. The `separation-distance` Feature

This feature was disabled due to its reliance on non-Rust dependencies that are incompatible with `wasm32-unknown-unknown`.

### 6.1 Why It Was Disabled

The feature uses geometric operations to ensure polygons maintain a certain minimum separation. However, its dependency stack introduces compatibility issues.

### 6.2 Dependencies and Their Issues

From `Cargo.toml`:

```toml
geo-offset = { version = "0.4.0", optional = true }
geo-types = { version = "0.7.16", optional = true }
```

`geo-offset` internally depends on `geo-clipper`, which in turn depends on `clipper-sys`, a Rust FFI wrapper for the C++ `Clipper` library.

### 6.3 Compiling C++ to WASM: Why It's Problematic

C++ code can only be compiled to WASM using **Emscripten**, which provides a C++ standard library in a WASM-compatible format. However:

* This cannot be used with the `wasm32-unknown-unknown` target.
* `wasm-bindgen` and `wasm-bindgen-rayon` are not compatible with Emscripten.
* Threading, memory sharing, and JS bindings would all need to be rewritten.

### 6.4 Why Emscripten Was Rejected

Using `wasm32-unknown-emscripten` would:

* Break `wasm-bindgen`-based integration.
* Prevent usage of `wasm-bindgen-rayon`.
* Require maintaining a dual toolchain and completely different build and integration workflow.

These trade-offs were deemed too costly for minimal functional gain.

### 6.5 Resolution

The preferred solution is to switch to a **pure Rust** alternative for polygon offsetting.

#### Suggested Alternative:

* [`geo-booleanop`](https://crates.io/crates/geo-booleanop) – performs geometric boolean operations and is compatible with `wasm32-unknown-unknown`.

## 7. Future Considerations

* Restore `SPP` logic behind a feature flag if ever needed.
* Evaluate switching to `geo-booleanop` or another WASM-safe crate to re-enable `separation-distance`.
* Consider refactoring logging to use `console_log` (or similar) for browser-friendly output.
* Long term: Modularize WASM vs native builds using conditional compilation (`cfg(target_arch = "wasm32")`) to keep build logic clean and safe.

