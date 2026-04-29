pub mod ore;

use crate::world::chunk::tile::ore::Ore;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Tile {
    pub ore: Option<Ore>,
}
