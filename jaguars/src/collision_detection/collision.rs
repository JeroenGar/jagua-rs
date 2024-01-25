#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Collides {
    No,
    Yes,
    Unsure
}

impl From<bool> for Collides {
    fn from(value: bool) -> Self {
        if value {
            Collides::Yes
        } else {
            Collides::No
        }
    }
}