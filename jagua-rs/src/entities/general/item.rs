use std::sync::Arc;

use crate::collision_detection::hazard_filter::QZHazardFilter;
use crate::geometry::geo_enums::AllowedRotation;
use crate::geometry::primitives::simple_polygon::SimplePolygon;
use crate::geometry::transformation::Transformation;
use crate::util::config::SPSurrogateConfig;

/// Item to be placed in a Layout
#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    /// Contour of the item
    pub shape: Arc<SimplePolygon>,
    /// Possible rotations in which to place the item
    pub allowed_rotation: AllowedRotation,
    /// The quality of the item, if `None` the item requires full quality
    pub base_quality: Option<usize>,
    pub value: u64,
    /// Transformation applied to the shape with respect to the original shape in the input file (for example to center it).
    pub pretransform: Transformation,
    /// Filter for hazards that the item is unaffected by
    pub hazard_filter: Option<QZHazardFilter>,
    /// Configuration for the surrogate generation
    pub surrogate_config: SPSurrogateConfig,
}

impl Item {
    pub fn new(
        id: usize,
        mut shape: SimplePolygon,
        allowed_rotation: AllowedRotation,
        base_quality: Option<usize>,
        value: u64,
        pretransform: Transformation,
        surrogate_config: SPSurrogateConfig,
    ) -> Item {
        shape.generate_surrogate(surrogate_config);
        let shape = Arc::new(shape);
        let hazard_filter = base_quality.map(QZHazardFilter);
        Item {
            id,
            shape,
            allowed_rotation,
            base_quality,
            value,
            pretransform,
            hazard_filter,
            surrogate_config,
        }
    }
}
