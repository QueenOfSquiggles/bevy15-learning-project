use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

pub const SECONDS_PER_TURN: f32 = 6.0;
pub const ACTIONS_PER_TURN: u32 = 3;
pub const SECONDS_PER_ACTION: f32 = SECONDS_PER_TURN / (ACTIONS_PER_TURN as f32);

#[derive(Component, Reflect, Clone, Serialize, Deserialize)]
pub struct CoreData {
    pub name: String,
    pub base_modifiers: HashMap<Stats, i16>,
    pub skill_levels: HashMap<Skills, TrainingLevel>,
    pub hp: Health,
    pub ac: ArmourClass,
    pub conditions: Vec<String>,
    pub speed: u32,
}

#[derive(Hash, Reflect, Clone, Serialize, Deserialize)]
pub struct ArmourClass(pub u32);

#[derive(Hash, Reflect, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: i32,
    pub max: u32,
}

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
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

impl Skills {
    pub fn associated_stat(&self) -> Stats {
        match *self {
            Skills::Acrobatics => Stats::Dexterity,
            Skills::Arcana => Stats::Intelligence,
            Skills::Athletics => Stats::Strength,
            Skills::Crafting => Stats::Intelligence,
            Skills::Deception => Stats::Charisma,
            Skills::Diplomacy => Stats::Charisma,
            Skills::Intimidation => Stats::Charisma,
            Skills::Lore => Stats::Intelligence,
            Skills::Medicine => Stats::Wisdom,
            Skills::Nature => Stats::Wisdom,
            Skills::Occultism => Stats::Intelligence,
            Skills::Performance => Stats::Charisma,
            Skills::Religion => Stats::Wisdom,
            Skills::Society => Stats::Intelligence,
            Skills::Stealth => Stats::Dexterity,
            Skills::Survival => Stats::Wisdom,
            Skills::Thievery => Stats::Dexterity,
        }
    }
}

#[derive(Hash, Reflect, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum TrainingLevel {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

impl TrainingLevel {
    pub fn get_modifier(&self) -> i16 {
        match *self {
            TrainingLevel::Untrained => 0,
            TrainingLevel::Trained => 2,
            TrainingLevel::Expert => 4,
            TrainingLevel::Master => 6,
            TrainingLevel::Legendary => 8,
        }
    }
}
