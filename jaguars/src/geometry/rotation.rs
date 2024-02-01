#[derive(Clone, Debug, PartialEq)]
pub enum Rotation{
    None,
    Continuous,
    Discrete(Vec<f64>)
}