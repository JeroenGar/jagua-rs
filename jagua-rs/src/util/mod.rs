/// Set of functions used throughout assure the correctness of the library.
pub mod assertions;

mod config;
mod fpa;
mod polygon_simplification;

#[doc(inline)]
pub use polygon_simplification::PolySimplConfig;
#[doc(inline)]
pub use polygon_simplification::PolySimplMode;
#[doc(inline)]
pub use polygon_simplification::simplify_poly;
#[doc(inline)]
pub use config::SPSurrogateConfig;
#[doc(inline)]
pub use config::CDEConfig;
#[doc(inline)]
pub use fpa::FPA;




