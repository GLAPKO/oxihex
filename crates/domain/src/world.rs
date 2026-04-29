pub mod chunk;
pub mod hex;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::world::chunk::Chunk;

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub chunks: HashMap<ChunkPos, Chunk>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Seed(pub u64);

impl Seed {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn inner(self) -> u64 {
        self.0
    }
}