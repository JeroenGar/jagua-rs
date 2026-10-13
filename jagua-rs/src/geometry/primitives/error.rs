use crate::geometry::primitives::Point;

/// Why a geometric primitive could not be constructed.
///
/// The constructors of [`SPolygon`](super::SPolygon), [`Rect`](super::Rect),
/// [`Circle`](super::Circle), and [`Edge`](super::Edge) return it inside an [`anyhow::Error`];
/// recover it with [`anyhow::Error::downcast_ref`], for example to point at the offending input.
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum GeometryError {
    #[error("simple polygon must have at least 3 points, it has {0}")]
    TooFewPoints(usize),
    /// The indices of two equal points.
    #[error("simple polygon contains duplicate points {0} and {1}")]
    DuplicatePoints(usize, usize),
    /// The indices of two edges that intersect; edge `i` runs from point `i` to point `i + 1`,
    /// and the last edge closes the polygon.
    #[error("simple polygon contains intersecting edges {0} and {1}")]
    IntersectingEdges(usize, usize),
    #[error("simple polygon has no area")]
    NoArea,
    #[error("invalid rectangle, x_min: {x_min}, x_max: {x_max}, y_min: {y_min}, y_max: {y_max}")]
    InvalidRect {
        x_min: f32,
        y_min: f32,
        x_max: f32,
        y_max: f32,
    },
    #[error("invalid circle, center: {center:?}, radius: {radius}")]
    InvalidCircle { center: Point, radius: f32 },
    #[error("degenerate edge, {0:?} == {0:?}")]
    DegenerateEdge(Point),
}
