// packaging all modules into a single large crate

#[doc(inline)]
pub use jagua_rs_base::*;

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
