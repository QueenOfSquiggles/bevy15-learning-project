use std::time::{Duration, Instant};

use crate::game_states::MouseState;

use super::{inputs::Inputs, CameraAxisNode, PlayerRoot};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use seldom_state::prelude::*;

const PLAYER_SPEED: f32 = 10.0;
const PLAYER_DODGE_SPEED: f32 = 20.0;

pub struct PlayerStatesPlugin;

impl Plugin for PlayerStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StateMachinePlugin);
        app.add_systems(
            Update,
            (player_state_move, player_state_dodge).run_if(in_state(MouseState::Captured)),
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

// #[derive(Component, Clone)]
// pub struct StateCutscene;

pub fn player_root_bundle() -> (StateMoving, StateMachine) {
    (
        StateMoving,
        StateMachine::default()
            .trans_builder(
                just_pressed(Inputs::Dodge).and(axis_pair_unbounded(Inputs::Move)),
                build_state_dodge,
            )
            .trans::<AnyState, _>(done(Some(Done::Success)), StateMoving),
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
        (&mut Transform, &ActionState<Inputs>),
        (With<PlayerRoot>, With<StateMoving>, Without<CameraAxisNode>),
    >,
    mut q_camera: Query<&mut Transform, (With<CameraAxisNode>, Without<PlayerRoot>)>,
    time: Res<Time>,
) {
    let Ok((mut trans, input)) = query.get_single_mut() else {
        return;
    };
    let Ok(mut cam_trans) = q_camera.get_single_mut() else {
        return;
    };
    let movement = input.axis_pair(&Inputs::Move);
    let look = input.axis_pair(&Inputs::Look);
    let offset = (trans.forward() * movement.y) + (trans.right() * movement.x);
    trans.translation += offset.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    trans.rotate_y(look.x * time.delta_secs());
    let (mut x, y, z) = cam_trans.rotation.to_euler(EulerRot::XYZ);
    x = (x + look.y * time.delta_secs()).clamp(-70.0_f32.to_radians(), 10.0_f32.to_radians());
    cam_trans.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);
}

fn player_state_dodge(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Transform, &StateDodge), With<PlayerRoot>>,
    time: Res<Time>,
) {
    let Ok((e, mut trans, dodge)) = query.get_single_mut() else {
        return;
    };
    if dodge.started.elapsed() > dodge.duration {
        cmd.entity(e).insert(Done::Success);
    } else {
        let offset = (trans.forward() * dodge.dir.z) + (trans.right() * dodge.dir.x);

        trans.translation += offset.normalize_or_zero() * PLAYER_DODGE_SPEED * time.delta_secs();
    }
}
