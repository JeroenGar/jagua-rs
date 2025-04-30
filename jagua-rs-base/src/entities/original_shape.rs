use crate::geometry::DTransformation;
use crate::geometry::geo_traits::{Shape, Transformable};
use crate::geometry::primitives::{Point, Rect, SPolygon};
use crate::geometry::shape_modification::{
    ShapeModifyConfig, ShapeModifyMode, offset_shape, simplify_shape,
};

#[derive(Clone, Debug)]
/// A [`SPolygon`] exactly as is defined in the input file
///
/// Also contains all required operation to convert it to a shape that can be used internally.
/// Currently, these are centering and simplification operations, but could be extended in the future.
pub struct OriginalShape {
    pub shape: SPolygon,
    pub pre_transform: DTransformation,
    pub modify_mode: ShapeModifyMode,
    pub modify_config: ShapeModifyConfig,
}

impl OriginalShape {
    pub fn convert_to_internal(&self) -> SPolygon {
        // Apply the transformation
        let mut internal = self.shape.transform_clone(&self.pre_transform.compose());

        if let Some(offset) = self.modify_config.offset {
            // Offset the shape
            internal = offset_shape(&internal, self.modify_mode, offset);
        }
        if let Some(tolerance) = self.modify_config.simplify_tolerance {
            // Simplify the shape
            internal = simplify_shape(&internal, self.modify_mode, tolerance);
        };
        internal
    }
}

impl Shape for OriginalShape {
    fn centroid(&self) -> Point {
        self.shape.centroid()
    }

    fn area(&self) -> f32 {
        self.shape.area()
    }

    fn bbox(&self) -> Rect {
        self.shape.bbox
    }

    fn diameter(&self) -> f32 {
        self.shape.diameter()
    }
}
