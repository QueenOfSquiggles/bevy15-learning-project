use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{
    color::palettes::css,
    core_pipeline::{
        contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
        experimental::taa::TemporalAntiAliasing,
    },
    prelude::*,
};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use inputs::PlayerInputsPlugin;
use states::PlayerStatesPlugin;

use crate::{
    items::ItemSlot,
    level::{EventEndLoadingLevel, EventStartLoadingLevel},
};

pub mod inputs;
pub mod states;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerInputsPlugin, PlayerStatesPlugin));
        app.add_systems(Startup, setup_player);
        app.add_observer(
            |_: Trigger<EventStartLoadingLevel>,
             mut cmd: Commands,
             q: Query<Entity, With<PlayerRoot>>| {
                let Ok(player) = q.get_single() else {
                    return;
                };
                cmd.entity(player).insert(LockedAxes::ALL_LOCKED);
            },
        );
        app.add_observer(
            |_: Trigger<EventEndLoadingLevel>,
             mut cmd: Commands,
             q: Query<Entity, With<PlayerRoot>>| {
                let Ok(player) = q.get_single() else {
                    return;
                };
                cmd.entity(player).insert(LockedAxes::ROTATION_LOCKED);
            },
        );
    }
}

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
struct CameraAxisNode;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Component, Default)]
pub struct PlayerEquipment {
    weapon: ItemSlot,
}

fn setup_player(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Name::new("Player"),
        PlayerRoot,
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.5))),
        MeshMaterial3d(mats.add(StandardMaterial::from_color(css::BLANCHED_ALMOND))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        inputs::player_root_bundle(), // add input management
        states::player_root_bundle(), // add states (components only)
        PlayerEquipment {
            weapon: ItemSlot(Some(assets.load("item/test_weapon.json"))),
        },
        RigidBody::Dynamic,
        Collider::capsule(0.3, 1.0),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        LockedAxes::ALL_LOCKED,
    ))
    .with_children(|cmd| {
        cmd.spawn((
            Name::new("Head"),
            Mesh3d(meshes.add(Cuboid::from_length(0.5))),
            MeshMaterial3d(mats.add(StandardMaterial::from_color(css::MISTY_ROSE))),
            Transform::from_translation(Vec3::new(0.0, 0.75, -0.2)),
        ));
        // cmd.spawn(PointLight {
        //     intensity: 100.0,
        //     range: 15.0,
        //     shadows_enabled: false,
        //     ..default()
        // });
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
                TemporalAntiAliasing::default(),
                ContrastAdaptiveSharpening::default(),
                Msaa::Off,
            ));
        });
    });
}
