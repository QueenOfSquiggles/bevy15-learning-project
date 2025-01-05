use std::time::{Duration, Instant};

use crate::{
    game_states::MouseState,
    items::{ItemType, WeaponItem},
};

use super::{inputs::Inputs, CameraAxisNode, PlayerEquipment, PlayerRoot};
use bevy::prelude::*;
use bevy_tnua::{
    prelude::{TnuaBuiltinWalk, TnuaController},
    TnuaUserControlsSystemSet,
};
use leafwing_input_manager::prelude::ActionState;
use seldom_state::prelude::*;

const TO_RADIANS: f32 = 3.14 / 180.0; // 2 * pi / 360_deg = pi / 180_deg

const PLAYER_SPEED: f32 = 10.0;
const PLAYER_DODGE_SPEED: f32 = 20.0;
const PLAYER_TURN_SPEED: f32 = 45.0 * TO_RADIANS;

pub struct PlayerStatesPlugin;

impl Plugin for PlayerStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (player_state_move, player_state_dodge, player_state_attack)
                .run_if(in_state(MouseState::Captured))
                .in_set(TnuaUserControlsSystemSet),
        );
    }
}

#[derive(Component, Clone)]
pub struct StateMoving;

#[derive(Component, Clone)]
pub struct StateDodge {
    dir: Vec3,
    started: Instant,
    duration: Duration,
}

#[derive(Component, Clone, Default)]
pub struct StateAttack {
    weapon: Option<WeaponItem>,
    time: Option<Timer>,
}

#[derive(Event)]
struct InitAttackDataEvent;

pub fn player_root_bundle() -> (StateMoving, StateMachine, Observer) {
    (
        StateMoving,
        StateMachine::default()
            .trans_builder(
                just_pressed(Inputs::Dodge).and(axis_pair_unbounded(Inputs::Move)),
                build_state_dodge,
            )
            .trans::<StateMoving, _>(just_pressed(Inputs::Attack), StateAttack::default())
            .trans::<StateAttack, _>(done(Some(Done::Success)), StateMoving)
            .trans::<StateDodge, _>(done(None), StateMoving)
            .trans::<AnyState, _>(done(Some(Done::Failure)), StateMoving) // fallback to moving on fail
            .on_enter::<StateAttack>(|e| {
                e.trigger(InitAttackDataEvent);
            }),
        Observer::new(init_state_attack),
    )
}

// Builders
fn build_state_dodge(_: &StateMoving, params: ((), Vec2)) -> Option<StateDodge> {
    let move_dir = params.1.normalize_or(Vec2::NEG_Y);
    Some(StateDodge {
        dir: Vec3::new(move_dir.x, 0.0, move_dir.y),
        started: Instant::now(),
        duration: Duration::from_secs_f32(0.5),
    })
}

// State Logic

fn player_state_move(
    mut query: Query<
        (&mut TnuaController, &mut Transform, &ActionState<Inputs>),
        (With<PlayerRoot>, With<StateMoving>, Without<CameraAxisNode>),
    >,
    mut q_camera: Query<&mut Transform, (With<CameraAxisNode>, Without<PlayerRoot>)>,
    time: Res<Time>,
) {
    let Ok((mut body, mut trans, input)) = query.get_single_mut() else {
        return;
    };
    let Ok(mut cam_trans) = q_camera.get_single_mut() else {
        return;
    };
    let movement = input.axis_pair(&Inputs::Move);
    let look = input.axis_pair(&Inputs::Look);
    let intended_velocity = ((trans.forward() * movement.y) + (trans.right() * movement.x))
        .normalize_or_zero()
        * PLAYER_SPEED;
    body.basis(TnuaBuiltinWalk {
        desired_velocity: intended_velocity,
        float_height: 1.5,
        ..default()
    });

    trans.rotate_y(look.x * PLAYER_TURN_SPEED * time.delta_secs());
    let (mut x, y, z) = cam_trans.rotation.to_euler(EulerRot::XYZ);
    x = (x + (look.y * PLAYER_TURN_SPEED * time.delta_secs()))
        .clamp(-70.0_f32.to_radians(), 10.0_f32.to_radians());
    cam_trans.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);
}

fn player_state_dodge(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut TnuaController, &Transform, &StateDodge), With<PlayerRoot>>,
) {
    let Ok((e, mut body, trans, dodge)) = query.get_single_mut() else {
        return;
    };
    if dodge.started.elapsed() > dodge.duration {
        cmd.entity(e).insert(Done::Success);
    } else {
        body.basis(TnuaBuiltinWalk {
            desired_velocity: ((trans.forward() * dodge.dir.z) + (trans.right() * dodge.dir.x))
                .normalize_or_zero()
                * PLAYER_DODGE_SPEED,
            float_height: 1.5,
            ..default()
        });
    }
}

fn init_state_attack(
    _: Trigger<InitAttackDataEvent>,
    items: Res<Assets<ItemType>>,
    mut q: Query<(Entity, &mut StateAttack, &PlayerEquipment)>,
    mut cmd: Commands,
) {
    let Ok((e, mut attack, equipment)) = q.get_single_mut() else {
        warn!("Failed to get needed entity/components for handling `InitAttackDataEvent`");
        return;
    };
    let Some(handle) = &equipment.weapon.0 else {
        cmd.entity(e).insert(Done::Failure);
        return;
    };
    let Some(ItemType::Weapon(weapon)) = items.get(handle.id()) else {
        cmd.entity(e).insert(Done::Failure);
        return;
    };
    attack.weapon = Some(weapon.clone());
    attack.time = Some(Timer::from_seconds(weapon.attack_duration, TimerMode::Once));
    info!("Starting attack");
}

fn player_state_attack(
    mut cmd: Commands,
    mut q: Query<(Entity, &mut StateAttack)>,
    time: Res<Time>,
) {
    let Ok((e, mut state)) = q.get_single_mut() else {
        return;
    };
    let Some(timer) = &mut state.time else {
        return;
    };
    timer.tick(time.delta());
    if timer.just_finished() {
        cmd.entity(e).insert(Done::Success);
    }
}
