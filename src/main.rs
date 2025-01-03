use bevy::{math::Affine2, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ext_asset_server::AssetServerExtensions;
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
            PlayerPlugin,
            GameStatesPlugin,
            ItemsPlugin,
            ToastPlugin,
            WorldInspectorPlugin::new(),
            SettingsPlugin,
        ))
        .add_systems(Startup, create_world)
        .add_systems(Update, quit_on_f8)
        .run();
}

fn create_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    cmd.spawn((
        DirectionalLight {
            illuminance: 1_000.0,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(0.5, -1.0, 0.5), Vec3::Y),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE * 32.0))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color_texture: Some(
                assets.load_image_custom("kenney_prototyping_textures/Dark/texture_01.png"),
            ),
            uv_transform: Affine2::from_scale(Vec2::ONE * 32.0),
            ..default()
        })),
    ));
}

fn quit_on_f8(buttons: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if buttons.any_just_pressed([KeyCode::F8]) {
        writer.send(AppExit::Success);
    }
}
