use crate::entities::problems::problem::LayoutIndex;
use crate::geometry::d_transformation::DTransformation;
use crate::geometry::transformation::Transformation;

#[derive(Clone, Debug)]
pub struct InsertionOption {
    layout_index: LayoutIndex,
    item_id: usize,
    transformation: Transformation,
    d_transformation: DTransformation,
}

impl InsertionOption {
    pub fn new(layout_index: LayoutIndex, item_id: usize, transformation: Transformation, d_transformation: DTransformation) -> Self {
        InsertionOption {
            layout_index,
            item_id,
            transformation,
            d_transformation,
        }
    }

    pub fn item_id(&self) -> usize {
        self.item_id
    }

    pub fn transformation(&self) -> &Transformation {
        &self.transformation
    }

    pub fn layout_index(&self) -> &LayoutIndex {
        &self.layout_index
    }

    pub fn d_transformation(&self) -> &DTransformation {
        &self.d_transformation
    }
}