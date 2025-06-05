# jagua-rs for WASM

> [!WARNING]
> 
> The `lbf` directory has been made to completely support **ONLY** `wasm32-unknown-unknown` target 
> and **cannot** be compiled like mentioned in the main repository. (using cargo)
> 

This particular fork of jagua-rs implements the algorithm for a WASM target (`wasm32-unknown-unknown` in particular)

## Building

```bash 
# in the root dir
./build-wasm.sh --target <provide-target>
```

Possible targets are **web** and **nodejs** for now.

An example:

```bash 
./build-wasm.sh --target web
```

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
