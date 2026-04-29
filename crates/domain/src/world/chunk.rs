use serde::{Deserialize, Serialize};

use crate::world::chunk::tile::Tile;

pub mod tile;

// shared/src/domain/world.rs

pub const CHUNK_SIZE: usize = 32;
// pub const CHUNK_TILES: usize = CHUNK_SIZE * CHUNK_SIZE; // 1024 hexů

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Chunk {
    // Používáme fixní pole místo Vec
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
}
