use slotmap::new_key_type;

pub mod problem;
pub mod solution;
pub mod instance;

new_key_type! {
    /// Unique key for each `Layout` in a `BPProblem` and `BPSolution`
    pub struct LayKey;
}