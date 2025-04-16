use crate::fsize;
use crate::geometry::DTransformation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::SimplePolygon;
use crate::util::{simplify_poly, PolySimplConfig, PolySimplMode};

#[derive(Clone, Debug)]
/// A [`SimplePolygon`] exactly as is defined in the input file
///
/// Also contains all required operation to convert it to a shape that can be used internally.
/// Currently these are centering and simplification operations, but could be extended in the future.
pub struct OriginalShape{
    pub original: SimplePolygon,
    pub pre_transform: DTransformation,
    pub simplification: (PolySimplConfig, PolySimplMode)
}

impl OriginalShape {
    pub fn convert_to_internal(&self) -> SimplePolygon {
        let t = self.pre_transform.compose();
        let internal = self.original.transform_clone(&t);
        match self.simplification.0 {
            PolySimplConfig::Disabled => internal,
            PolySimplConfig::Enabled{tolerance} => {
                simplify_poly(&internal, self.simplification.1, tolerance)
            }
        }
    }

    pub fn area(&self) -> fsize {
        self.original.area()
    }
}