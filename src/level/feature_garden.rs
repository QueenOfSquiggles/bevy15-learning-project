use std::time::Duration;

use super::{LevelDescription, LevelState};
use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::{Audio, AudioControl};

pub struct LevelFeatureGarden;

impl LevelDescription<LevelState> for LevelFeatureGarden {
    type LevelAssets = GardenAssets;
    const LOAD_STATE: LevelState = LevelState::LoadFeatureGarden;
    const LEVEL_STATE: LevelState = LevelState::PlayFeatureGarden;
    const ADDITIONAL_SETUP: Option<fn(&mut App)> = Some(setup);
}
fn setup(app: &mut App) {
    app.add_systems(OnEnter(LevelState::PlayFeatureGarden), load_level);
}
fn load_level(
    mut cmd: Commands,
    assets: Res<GardenAssets>,
    gltf: Res<Assets<Gltf>>,
    audio: Res<Audio>,
) {
    let Some(scene) = gltf
        .get(assets.level.id())
        .and_then(|g| g.default_scene.clone())
    else {
        return;
    };

    cmd.spawn((
        Name::new("Level Root"),
        SceneRoot(scene),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
    ));
    audio
        .play(assets.bgm.clone_weak())
        .looped()
        .linear_fade_in(Duration::from_secs_f32(0.5))
        .with_volume(0.5);
}

#[derive(AssetCollection, Resource, Clone)]
pub struct GardenAssets {
    #[asset(path = "level/feature_garden.glb")]
    level: Handle<Gltf>,
    #[asset(path = "kenney_audio/music_loops/Sad Town.ogg")]
    bgm: Handle<bevy_kira_audio::AudioSource>,
}
