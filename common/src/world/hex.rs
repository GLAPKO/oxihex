pub use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn distance(a: Hex, b: Hex) -> i32 {
        ((a.q - b.q).abs() + (a.r - b.r).abs() + (a.s() - b.s()).abs()) / 2
    }
}