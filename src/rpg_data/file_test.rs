use std::fs::File;

use bevy::utils::hashbrown::HashMap;

use super::{
    core::{ArmourClass, CoreData, Health, SkillEntry, Skills, Stats, TrainingLevel},
    npc::{NpcCombatData, NpcNoncombatData},
    CharacterData, CharacterType,
};

pub fn test_serialize_character_asset() {
    let mut skills = HashMap::new();
    skills.insert(
        Skills::Arcana,
        SkillEntry(Stats::Wisdom, TrainingLevel::Trained),
    );
    let stats = HashMap::from_iter([(Stats::Charisma, -2), (Stats::Dexterity, 3)]);
    let data = CharacterData {
        core: CoreData {
            stats,
            skills,
            hp: Health {
                current: 15,
                max: 20,
            },
            ac: ArmourClass(15),
            conditions: vec!["flatfoot".to_owned()],
        },
        char_type: CharacterType::NpcVersatile(
            NpcCombatData { temp: 5 },
            NpcNoncombatData { temp: 10 },
        ),
    };

    let Ok(file) = File::create("assets/character/test_char.json") else {
        return;
    };
    let _ = serde_json::to_writer_pretty(file, &data);
}
