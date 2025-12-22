mod export;
mod import;

/// External (serializable) representations of all Strip Packing Problem related entities.
pub mod ext_repr;

/// Exports a strip packing solution out of the library.
pub use export::export;

/// Imports a strip packing instance into the library.
pub use import::import;
