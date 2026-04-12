mod chunk;
mod hex;

// shared/src/domain/world.rs

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::world::chunk::Chunk;

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    // Mapa chunků: klíčem je (ChunkQ, ChunkR)
    pub chunks: HashMap<(i32, i32), Chunk>,
}