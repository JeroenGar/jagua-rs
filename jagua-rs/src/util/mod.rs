/// Set of functions used throughout assure the correctness of the library.
pub mod assertions;

mod config;
mod fpa;
mod polygon_modification;

#[doc(inline)]
pub use config::CDEConfig;
#[doc(inline)]
pub use config::SPSurrogateConfig;
#[doc(inline)]
pub use fpa::FPA;
#[doc(inline)]
pub use polygon_modification::ShapeModifyConfig;
#[doc(inline)]
pub use polygon_modification::ShapeModifyMode;
#[doc(inline)]
pub use polygon_modification::offset_shape;
#[doc(inline)]
pub use polygon_modification::simplify_shape;
