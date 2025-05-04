//!
//! A fast and fearless Collision Detection Engine for 2D irregular cutting and packing problems.
//!
//! This library is designed to be used as a backend by optimization algorithms.
//!
//! All logic agnostic to a specific problem variant is implemented in the `jagua-rs-base` module.
//! Specific problem variants can be added by enabling the corresponding feature flag in your `Cargo.toml`.
//!
#![doc = document_features::document_features!()]

#[doc(inline)]
pub use jagua_rs_base::*;

/// Enabled variants of the 2D irregular Cutting and Packing Problem.
pub mod probs {
    /// Strip Packing Problem (SPP) module for `jagua-rs`.
    #[cfg(feature = "spp")]
    pub mod spp {
        #[doc(inline)]
        pub use jagua_rs_spp::*;
    }

    /// Bin Packing Problem (BPP) module for `jagua-rs`.
    #[cfg(feature = "bpp")]
    pub mod bpp {
        #[doc(inline)]
        pub use jagua_rs_bpp::*;
    }
}
