use std::num::NonZeroU8;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlayerId(pub NonZeroU8);

impl PlayerId {
    pub fn new(id: u8) -> Option<Self> {
        NonZeroU8::new(id).map(PlayerId)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player{
    pub id: PlayerId,
    pub nick: String,
}