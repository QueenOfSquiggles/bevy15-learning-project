use std::time::Duration;

use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{
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
    player,
};

pub mod inputs;
pub mod states;

pub const PLAYER_HEIGHT: f32 = 1.75;
pub const PLAYER_COLLIDER_FLOAT_HEIGHT: f32 = 0.5;
pub const PLAYER_COLLIDER_HEIGHT: f32 = PLAYER_HEIGHT - PLAYER_COLLIDER_FLOAT_HEIGHT;
pub const PLAYER_RADIUS: f32 = 0.3;
/// Used for the collider since bevy asks for the cylinder height and appends the hemisphere caps on top of that
pub const PLAYER_COLLIDER_LENGTH: f32 = PLAYER_COLLIDER_HEIGHT - (PLAYER_RADIUS * 2.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerInputsPlugin, PlayerStatesPlugin));
        app.init_resource::<PlayerAnimations>();
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, start_idle_anim);
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
pub struct PlayerModel;
#[derive(Component)]
struct CameraAxisNode;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Component, Default)]
pub struct PlayerEquipment {
    weapon: ItemSlot,
}

#[derive(Resource, Default)]
pub struct PlayerAnimations {
    animations: Vec<AnimationNodeIndex>,
    graph: Option<Handle<AnimationGraph>>,
    // idle: Option<Handle<AnimationClip>>,
    // run: Option<Handle<AnimationClip>>,
    // crouch_block: Option<Handle<AnimationClip>>,
    // death_keel_over: Option<Handle<AnimationClip>>,
    // death_fall_back: Option<Handle<AnimationClip>>,
    // slash: Option<Handle<AnimationClip>>,
}

fn setup_player(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut anim_graphs: ResMut<Assets<AnimationGraph>>,
    mut player_anim: ResMut<PlayerAnimations>,
) {
    const MODEL: &str = "model/character/mixamo_char_testing.glb";
    let (graph, indices) = AnimationGraph::from_clips([
        assets.load(GltfAssetLabel::Animation(0).from_asset(MODEL)),
        assets.load(GltfAssetLabel::Animation(1).from_asset(MODEL)),
        assets.load(GltfAssetLabel::Animation(2).from_asset(MODEL)),
        assets.load(GltfAssetLabel::Animation(3).from_asset(MODEL)),
        assets.load(GltfAssetLabel::Animation(4).from_asset(MODEL)),
        assets.load(GltfAssetLabel::Animation(5).from_asset(MODEL)),
    ]);
    let graph_handle = anim_graphs.add(graph);
    player_anim.animations = indices;
    player_anim.graph = Some(graph_handle);

    cmd.spawn((
        Name::new("Player"),
        PlayerRoot,
        Transform::from_xyz(0.0, 2.0, 0.0),
        inputs::player_root_bundle(), // add input management
        states::player_root_bundle(), // add states (components only)
        PlayerEquipment {
            weapon: ItemSlot(Some(assets.load("item/test_weapon.json"))),
        },
        RigidBody::Dynamic,
        Collider::capsule(PLAYER_RADIUS, PLAYER_COLLIDER_LENGTH),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_RADIUS, 0.0)),
        LockedAxes::ALL_LOCKED,
        InheritedVisibility::default(),
    ))
    .with_children(|cmd| {
        cmd.spawn((
            PlayerModel,
            InheritedVisibility::default(),
            SceneRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(MODEL))),
            Transform::from_xyz(0.0, -PLAYER_COLLIDER_HEIGHT / 2.0, 0.0),
        ));
        cmd.spawn((
            Transform::from_translation(Vec3::Y * PLAYER_COLLIDER_HEIGHT / 2.0),
            InheritedVisibility::default(),
            CameraAxisNode,
        ))
        .with_children(|cmd| {
            cmd.spawn((
                Camera3d::default(),
                Transform::from_translation(Vec3::Z * 10.0),
                MainCamera,
                TemporalAntiAliasing::default(),
                ContrastAdaptiveSharpening::default(),
                Msaa::Off,
            ));
        });
    });
}

fn start_idle_anim(
    mut cmd: Commands,
    mut q_anim: Query<(Entity, &mut AnimationPlayer), (With<PlayerModel>, Added<AnimationPlayer>)>,
    player_anims: Res<PlayerAnimations>,
    // graphs: Res<Assets<AnimationGraph>>,
) {
    let Ok((entity, mut player)) = q_anim.get_single_mut() else {
        return;
    };
    let Some(player_graph) = &player_anims.graph else {
        warn!("failed to get animation graph");
        return;
    };
    // let Some(graph) = graphs.get(player_graph) else {
    //     return;
    // };
    let Some(idle_anim_index) = player_anims.animations.get(3) else {
        warn!(
            "failed to get animation index! Available: {:?}",
            player_anims.animations
        );
        return;
    };
    // let Some(node) = graph.get(*idle_anim_index) else {
    //     return;
    // };
    // let opt_clip = match &node.node_type {
    //     AnimationNodeType::Clip(handle) => Some(handle),
    //     _ => None,
    // };
    // let Some(clip) = opt_clip else {
    //     return;
    // };
    let mut trans = AnimationTransitions::new();
    trans
        .play(&mut player, *idle_anim_index, Duration::ZERO)
        .repeat();
    cmd.entity(entity)
        .insert((AnimationGraphHandle(player_graph.clone()), trans));
}
