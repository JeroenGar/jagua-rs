mod circle;
mod edge;
mod point;
mod rect;
mod polygon;

#[doc(inline)]
pub use circle::Circle;
#[doc(inline)]
pub use edge::Edge;
#[doc(inline)]
pub use point::Point;
#[doc(inline)]
pub use rect::Rect;
#[doc(inline)]
pub use polygon::Polygon;
#[doc(inline)]
pub use polygon::nonsimple_polygon::NSPolygon;
#[doc(inline)]
pub use polygon::simple_polygon::SPolygon;
