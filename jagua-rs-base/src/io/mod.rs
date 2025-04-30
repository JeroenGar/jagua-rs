/// External (serializable) representations of the entities within the library.
mod ext_repr;

/// All logic for converting external representations into internal ones
pub mod import;

/// All logic for exporting internal representations into external ones
pub mod export;
