#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Collides {
    Yes,
    No,
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