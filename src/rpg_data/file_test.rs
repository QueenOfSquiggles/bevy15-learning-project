use std::fs::File;

use bevy::utils::hashbrown::HashMap;

use super::{
    core::{ArmourClass, CoreData, Health, Skills, Stats, TrainingLevel},
    npc::{NpcCombatData, NpcNoncombatData},
    CharacterData, CharacterType,
};

pub fn test_serialize_character_asset() {
    let mut skills = HashMap::new();
    skills.insert(Skills::Arcana, TrainingLevel::Trained);
    let stats = HashMap::from_iter([(Stats::Charisma, -2), (Stats::Dexterity, 3)]);
    let data = CharacterData {
        core: CoreData {
            name: "Test Character".to_owned(),
            base_modifiers: stats,
            skill_levels: skills,
            hp: Health {
                current: 15,
                max: 20,
            },
            ac: ArmourClass(15),
            conditions: vec!["flatfoot".to_owned()],
            speed: 25,
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

pub fn test_character_valeros() {
    // Valeros is a pregen Human-Fighter character made for "Menace Under Otari"
    let data = CharacterData {
        core: CoreData {
            name: "Valeros".to_owned(),
            base_modifiers: HashMap::from_iter([
                (Stats::Strength, 4),
                (Stats::Dexterity, 2),
                (Stats::Constitution, 2),
                (Stats::Intelligence, 1),
            ]),
            hp: Health {
                current: 25,
                max: 25,
            },
            ac: ArmourClass(18),
            conditions: vec![],
            speed: 25,
            skill_levels: HashMap::from_iter([
                (Skills::Acrobatics, TrainingLevel::Trained),
                (Skills::Athletics, TrainingLevel::Trained),
                (Skills::Diplomacy, TrainingLevel::Trained),
                (Skills::Intimidation, TrainingLevel::Trained),
                (Skills::Lore, TrainingLevel::Trained),
                (Skills::Survival, TrainingLevel::Trained),
            ]),
        },
        char_type: CharacterType::NpcVersatile(
            NpcCombatData { temp: 5 },
            NpcNoncombatData { temp: 10 },
        ),
    };

    let Ok(file) = File::create("assets/character/valeros.json") else {
        return;
    };
    let _ = serde_json::to_writer_pretty(file, &data);
}
