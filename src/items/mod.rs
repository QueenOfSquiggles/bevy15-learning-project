use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use load_test::LoadTestPlugin;
use serde::{de::Error, Deserialize, Serialize};

mod load_test;
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemType>();
        app.init_asset_loader::<ItemAssetLoader>();
        app.add_plugins(LoadTestPlugin);
    }
}

#[derive(Default)]
pub struct ItemAssetLoader;

impl AssetLoader for ItemAssetLoader {
    type Asset = ItemType;
    type Settings = ();
    type Error = serde_json::Error;

    async fn load<'a>(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'a>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buffer = String::new();
        if let Err(e) = reader.read_to_string(&mut buffer).await {
            error!("Failed to load ");
            return Err(serde_json::Error::custom(format!(
                "Error with asset reader: {e}"
            )));
        }
        serde_json::from_str::<Self::Asset>(buffer.as_str())
    }
    fn extensions(&self) -> &[&str] {
        &[".json"]
    }
}
pub trait Item: Asset + Reflect + Clone + PartialEq {
    fn get_name(&self) -> &String;
}

#[derive(Debug, Asset, Reflect, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Basic(BasicItem),
    Weapon(WeaponItem),
    // A boxed value does not impl Send which is necessary here
}

#[derive(Debug, Asset, Reflect, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicItem {
    name: String,
}

#[derive(Debug, Asset, Reflect, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponItem {
    name: String,
    damage: u32,
    durability: u32,
}

impl Item for ItemType {
    fn get_name(&self) -> &String {
        match self {
            ItemType::Basic(basic_item) => basic_item.get_name(),
            ItemType::Weapon(weapon_item) => weapon_item.get_name(),
        }
    }
}

impl Item for BasicItem {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Item for WeaponItem {
    fn get_name(&self) -> &String {
        &self.name
    }
}
