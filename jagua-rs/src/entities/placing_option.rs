use crate::entities::problems::problem_generic::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
/// Encapsulates all required information to place an `Item` in a `Problem`
pub struct PlacingOption {
    pub layout_index: LayoutIndex,
    pub item_id: usize,
    pub transform: Transformation,
    pub d_transform: DTransformation,
}

impl PlacingOption {
    pub fn from_transform(
        layout_index: LayoutIndex,
        item_id: usize,
        transform: Transformation,
    ) -> Self {
        let d_transform = transform.decompose();
        PlacingOption {
            layout_index,
            item_id,
            transform,
            d_transform,
        }
    }

    pub fn from_d_transform(
        layout_index: LayoutIndex,
        item_id: usize,
        d_transform: DTransformation,
    ) -> Self {
        let transform = d_transform.compose();
        PlacingOption {
            layout_index,
            item_id,
            transform,
            d_transform,
        }
    }
}
