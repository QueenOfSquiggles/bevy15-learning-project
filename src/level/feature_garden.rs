use super::{LevelDescription, LevelState};
use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

pub struct LevelFeatureGarden;

impl LevelDescription<LevelState> for LevelFeatureGarden {
    type LevelAssets = GardenAssets;
    const LOAD_STATE: LevelState = LevelState::LoadFeatureGarden;
    const LEVEL_STATE: LevelState = LevelState::PlayFeatureGarden;
    const ENTER_SYSTEM: fn(&mut World) = load_level;
}

fn load_level(world: &mut World) {
    info!("Level asset collection finished loading!");
    let Some(assets) = world.get_resource::<GardenAssets>().cloned() else {
        return;
    };
    let Some(gltf) = world.get_resource::<Assets<Gltf>>() else {
        return;
    };
    let Some(scene) = gltf
        .get(assets.level.id())
        .and_then(|g| g.default_scene.clone())
    else {
        return;
    };

    let mut cmd = world.commands();
    cmd.spawn(Name::new("Test Level Init"));
    cmd.spawn((
        Name::new("Level Root"),
        SceneRoot(scene),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    ));
}

#[derive(AssetCollection, Resource, Clone)]
pub struct GardenAssets {
    #[asset(path = "level/feature_garden.glb")]
    level: Handle<Gltf>,
}
