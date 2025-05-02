// Packaging all modules into a single large crate with features

#[doc(inline)]
pub use jagua_rs_base::*;

/// Variants of 2D irregular Cutting and Packing Problems
pub mod problem_modules {
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
