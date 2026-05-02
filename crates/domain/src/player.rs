use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(NonZeroU8);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PlayerIdError {
    #[error("player id must be non-zero")]
    Zero,
}

impl PlayerId {
    pub fn get(self) -> u8 {
        self.0.get()
    }
}

impl From<PlayerId> for u8 {
    fn from(id: PlayerId) -> Self {
        id.get()
    }
}

// impl From<PlayerId> for NonZeroU8 {
//     fn from(id: PlayerId) -> Self {
//         id.0
//     }
// }

impl TryFrom<u8> for PlayerId {
    type Error = PlayerIdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        NonZeroU8::new(value)
            .map(PlayerId)
            .ok_or(PlayerIdError::Zero)
    }
}

// impl From<NonZeroU8> for PlayerId {
//     fn from(value: NonZeroU8) -> Self {
//         Self(value)
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    id: PlayerId,
    nick: String,
}

impl Player {
    pub fn new(id: PlayerId, nick: impl Into<String>) -> Self {
        Self {
            id,
            nick: nick.into(),
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn nick(&self) -> &str {
        &self.nick
    }
}