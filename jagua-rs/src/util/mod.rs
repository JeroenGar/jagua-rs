/// Set of functions used throughout assure the correctness of the library.
pub mod assertions;
mod fpa;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use fpa::FPA;
