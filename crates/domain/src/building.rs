use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BuildingId(NonZeroU64);

impl BuildingId {
    pub fn new(id: u64) -> Self {
        Self(NonZeroU64::new(id).unwrap())
    }

    pub fn try_new(id: u64) -> Option<Self> {
        NonZeroU64::new(id).map(Self)
    }

    pub fn get(self) -> u64 {
        self.0.get()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BuildingType {
    Conveyor,

}


#[derive(Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: BuildingId,

}