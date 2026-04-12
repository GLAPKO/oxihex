//mod Building;

use serde::{Deserialize, Serialize};
use crate::player::PlayerId;

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Tile {
    pub owner: Option<PlayerId>,
}