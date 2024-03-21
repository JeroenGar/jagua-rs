use crate::entities::instances::bin_packing::BPInstance;
use crate::entities::instances::instance_generic::InstanceGeneric;
use crate::entities::instances::strip_packing::SPInstance;
use crate::entities::item::Item;
use crate::fsize;

/// An `Instance` is the static (unmodifiable) representation of a problem instance.
/// This enum contains all variants of an instance.
/// See [`crate::entities::problems::problem::Problem`] for more information about the choice to represent variants as enums.
#[derive(Debug, Clone)]
pub enum Instance {
    SP(SPInstance),
    BP(BPInstance),
}

impl InstanceGeneric for Instance {
    fn items(&self) -> &[(Item, usize)] {
        match self {
            Instance::SP(instance) => instance.items(),
            Instance::BP(instance) => instance.items(),
        }
    }

    fn item_area(&self) -> fsize {
        match self {
            Instance::SP(instance) => instance.item_area(),
            Instance::BP(instance) => instance.item_area(),
        }
    }
}

impl From<SPInstance> for Instance {
    fn from(instance: SPInstance) -> Self {
        Instance::SP(instance)
    }
}

impl From<BPInstance> for Instance {
    fn from(instance: BPInstance) -> Self {
        Instance::BP(instance)
    }
}
