use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Clone, Serialize, Deserialize)]
pub struct CoreData {
    pub stats: HashMap<Stats, i16>,
    pub skills: HashMap<Skills, SkillEntry>,
    pub hp: Health,
    pub ac: ArmourClass,
    pub conditions: Vec<String>,
}

#[derive(Hash, Reflect, Clone, Serialize, Deserialize)]
pub struct ArmourClass(pub u32);

#[derive(Hash, Reflect, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: i32,
    pub max: u32,
}

#[derive(Hash, Reflect, Clone, Serialize, Deserialize)]
pub struct SkillEntry(pub Stats, pub TrainingLevel);

#[derive(Hash, Reflect, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum Stats {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Hash, Reflect, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum Skills {
    Acrobatics,
    Arcana,
    // TODO: fill remaining
    // TODO: create mapping from skill to core stat used
}

#[derive(Hash, Reflect, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum TrainingLevel {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}
