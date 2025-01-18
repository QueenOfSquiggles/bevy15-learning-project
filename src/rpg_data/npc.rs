use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Hash, Clone, Serialize, Deserialize)]
pub struct NpcCombatData {
    pub temp: u32,
}

#[derive(Reflect, Hash, Clone, Serialize, Deserialize)]
pub struct NpcNoncombatData {
    pub temp: u32,
}
