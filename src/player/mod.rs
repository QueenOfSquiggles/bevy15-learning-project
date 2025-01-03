use bevy::prelude::*;
use inputs::PlayerInputsPlugin;
use states::PlayerStatesPlugin;

mod inputs;
mod states;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerInputsPlugin, PlayerStatesPlugin));
        app.add_systems(Startup, setup_player);
    }
}

#[derive(Component)]
struct PlayerRoot;
#[derive(Component)]
struct CameraAxisNode;
#[derive(Component)]
struct MainCamera;

fn setup_player(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Name::new("Player"),
        PlayerRoot,
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.5))),
        MeshMaterial3d(mats.add(StandardMaterial::from_color(Color::linear_rgb(
            0.3, 0.3, 0.3,
        )))),
        Transform::from_translation(Vec3::Y * 0.75),
        inputs::player_root_bundle(), // add input management
        states::player_root_bundle(), // add states (components only)
    ))
    .with_children(|cmd| {
        cmd.spawn((
            Name::new("Head"),
            Mesh3d(meshes.add(Cuboid::from_length(0.5))),
            MeshMaterial3d(mats.add(StandardMaterial::from_color(Color::linear_rgb(
                1.0, 0.0, 0.0,
            )))),
            Transform::from_translation(Vec3::new(0.0, 0.75, -0.2)),
        ));
        cmd.spawn((
            Transform::from_translation(Vec3::Y * 0.75),
            InheritedVisibility::default(),
            CameraAxisNode,
        ))
        .with_children(|cmd| {
            cmd.spawn((
                Camera3d::default(),
                Transform::from_translation(Vec3::Z * 5.0),
                MainCamera,
            ));
        });
    });
}
