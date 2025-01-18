use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Hash, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
}
