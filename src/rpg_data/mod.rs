use core::CoreData;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use npc::{NpcCombatData, NpcNoncombatData};
use player::PlayerData;
use serde::{Deserialize, Serialize};

pub mod core;
pub mod file_test;
pub mod npc;
pub mod player;

pub struct RpgDataPlugin;

impl Plugin for RpgDataPlugin {
    fn build(&self, app: &mut App) {
        file_test::test_serialize_character_asset();
        file_test::test_character_valeros();
        app.register_asset_loader(CharacterDataAssetLoader);
    }
}

#[derive(Asset, Reflect, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub core: CoreData,
    pub char_type: CharacterType,
}

#[derive(Reflect, Hash, Clone, Serialize, Deserialize)]
pub enum CharacterType {
    Player(PlayerData),
    NpcCombat(NpcCombatData),
    NpcNoncombat(NpcNoncombatData),
    NpcVersatile(NpcCombatData, NpcNoncombatData),
}

pub struct CharacterDataAssetLoader;

impl AssetLoader for CharacterDataAssetLoader {
    type Asset = CharacterData;
    type Settings = ();
    type Error = serde_json::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _: &Self::Settings,
        _: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer).await;
        match serde_json::from_str::<CharacterData>(buffer.as_str()) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
