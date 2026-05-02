use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildingId(NonZeroU64);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BuildingIdError {
    #[error("Building id must be non-zero")]
    Zero,
}

impl BuildingId {
    pub fn get(self) -> u64 {
        self.0.get()
    }
}


impl From<BuildingId> for u64 {
    fn from(id: BuildingId) -> Self {
        id.get()
    }
}

impl TryFrom<u64> for BuildingId {
    type Error = BuildingIdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        NonZeroU64::new(value)
            .map(BuildingId)
            .ok_or(BuildingIdError::Zero)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: BuildingId,
}

impl Building {
    pub fn new(id: BuildingId) -> Self {
        Self {
            id,
        }
    }

    pub fn id(&self) -> BuildingId {
        self.id
    }
}