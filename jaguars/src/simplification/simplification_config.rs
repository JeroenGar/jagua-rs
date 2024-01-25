use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PolySimplConfig {
    Disabled,
    Enabled {
        tolerance: f64, //max deviation from the original polygon area (in %)
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PolySimplMode {
    Inflate,
    Deflate,
}

impl PolySimplMode {
    pub fn flip(&self) -> Self {
        match self {
            Self::Inflate => Self::Deflate,
            Self::Deflate => Self::Inflate,
        }
    }
}