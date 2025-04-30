mod export;
mod import;

///External (serializable) representations of the entities within the library.
pub mod ext_repr;

/// Exports a strip packing solution to an external representation.
pub use export::export;

/// Imports a strip packing instance from an external representation.
pub use import::import;
