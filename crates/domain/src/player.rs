use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlayerId(NonZeroU8);

impl PlayerId {
    pub fn new(id: u8) -> Self {
        Self(NonZeroU8::new(id).unwrap())
    }

    pub fn try_new(id: u8) -> Option<Self> {
        NonZeroU8::new(id).map(Self)
    }

    pub fn get(self) -> u8 {
        self.0.get()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub nick: String,
}
