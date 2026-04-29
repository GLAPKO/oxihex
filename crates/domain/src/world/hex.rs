pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pos {
    pub q: i32,
    pub r: i32,
}

impl Pos {
    pub const fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn distance(self, other: Pos) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s() - other.s()).abs()) / 2
    }

    pub fn neighbor(self, dir: Dir) -> Pos {
        let d = dir.to_pos();
        Pos {
            q: self.q + d.q,
            r: self.r + d.r,
        }
    }

    pub const ORIGIN: Pos = Pos { q: 0, r: 0 };
}

use std::ops::{Add, Sub};

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, rhs: i32) -> Self::Output {
        Pos {
            q: self.q * rhs,
            r: self.r * rhs,
        }
    }
}

impl std::ops::Neg for Pos {
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Pos {
            q: -self.q,
            r: -self.r,
        }
    }
}

impl From<Dir> for Pos {
    fn from(dir: Dir) -> Self {
        dir.to_pos()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dir {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl Dir {
    pub const fn to_pos(self) -> Pos {
        match self {
            Dir::NorthEast => Pos { q: 1, r: -1 },
            Dir::East => Pos { q: 1, r: 0 },
            Dir::SouthEast => Pos { q: 0, r: 1 },
            Dir::SouthWest => Pos { q: -1, r: 1 },
            Dir::West => Pos { q: -1, r: 0 },
            Dir::NorthWest => Pos { q: 0, r: -1 },
        }
    }

    pub const ALL: [Dir; 6] = [
        Dir::NorthEast,
        Dir::East,
        Dir::SouthEast,
        Dir::SouthWest,
        Dir::West,
        Dir::NorthWest,
    ];
}
