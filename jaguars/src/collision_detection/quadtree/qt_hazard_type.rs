use crate::collision_detection::quadtree::qt_partial_hazard::QTPartialHazard;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QTHazType {
    Partial(QTPartialHazard),
    Entire,
}