use crate::entities::Container;

#[derive(Debug, Clone)]
/// A container in the Bin Packing Problem (BPP) with an associated cost and stock.
pub struct Bin {
    /// Dense index in the instance's bin list.
    pub idx: usize,
    /// Caller-supplied bin type identifier.
    pub external_id: u64,
    /// The container in which to pack the items
    pub container: Container,
    /// The number of copies of this bin available to be use
    pub stock: usize,
    /// The cost of using a bin of this type
    pub cost: u64,
}

impl Bin {
    /// Creates a new bin with the given id, container, stock, and cost.
    #[must_use]
    pub fn new(
        idx: usize,
        external_id: u64,
        container: Container,
        stock: usize,
        cost: u64,
    ) -> Self {
        Self {
            idx,
            external_id,
            container,
            stock,
            cost,
        }
    }
}
