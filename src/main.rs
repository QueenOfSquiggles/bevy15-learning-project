use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin, pbr::CascadeShadowConfig, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_states::GameStatesPlugin;
use items::ItemsPlugin;
use player::PlayerPlugin;
use settings::SettingsPlugin;
use toast::ToastPlugin;

mod ext_asset_server;
mod game_states;
mod items;
mod player;
mod settings;
mod toast;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TemporalAntiAliasPlugin,
            PlayerPlugin,
            GameStatesPlugin,
            ItemsPlugin,
            ToastPlugin,
            WorldInspectorPlugin::new(),
            SettingsPlugin,
        ))
        .add_systems(Startup, load_level)
        .add_systems(Update, quit_on_f8)
        .run();
}

fn load_level(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Name::new("Level Root"),
        SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("level/feature_garden.glb"))),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
    ));
    cmd.spawn((
        DirectionalLight {
            illuminance: 1_000.0,
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfig {
            minimum_distance: 15.0,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(0.5, -1.0, -0.2), Vec3::Y),
    ));
}

fn quit_on_f8(buttons: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if buttons.any_just_pressed([KeyCode::F8]) {
        writer.send(AppExit::Success);
    }
}
