use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::settings::GameSettings;

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_toast);
        app.add_systems(Update, (update_toasts, test_toasts));
        app.add_observer(register_new_toasts);
    }
}

#[derive(Event)]
pub struct ToastEvent(pub String);

#[derive(Component)]
struct ToastRoot;

#[derive(Component)]
struct ToastItem {
    timer: Timer,
}

fn setup_toast(mut cmd: Commands) {
    cmd.spawn((
        ToastRoot,
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        Transform::from_translation(Vec3::new(10.0, 10.0, 0.0)),
    ));
}

fn register_new_toasts(
    trigger: Trigger<ToastEvent>,
    query: Query<Entity, With<ToastRoot>>,
    mut cmd: Commands,
    assets: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let Ok(root) = query.get_single() else { return };
    cmd.entity(root).with_child((
        ToastItem {
            timer: Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once),
        },
        Text::new(trigger.event().0.clone()),
        TextFont {
            font: assets.load(settings.font.clone()),
            font_size: 10.0,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.4)),
        BorderColor(Color::linear_rgb(0.0, 0.0, 0.0)),
        BorderRadius::all(Val::Percent(15.0)),
    ));
}

fn update_toasts(
    q_root: Query<&Children, With<ToastRoot>>,
    mut q_trans: Query<(Entity, &mut Transform, &mut ToastItem)>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    let Ok(children) = q_root.get_single() else {
        return;
    };
    for (index, child) in children.iter().enumerate() {
        let Ok((e, mut trans, mut toast)) = q_trans.get_mut(*child) else {
            continue;
        };
        toast.timer.tick(time.delta());
        if toast.timer.just_finished() {
            cmd.entity(e).despawn();
        }
        trans.translation = Vec3::lerp(
            trans.translation.clone(),
            Vec3::new(16.0, 30.0 * (index as f32), 0.0),
            0.3 * time.delta_secs(),
        );
    }
}

fn test_toasts(keyboard: Res<ButtonInput<KeyCode>>, mut cmd: Commands) {
    if keyboard.just_pressed(KeyCode::Enter) {
        cmd.trigger(ToastEvent(format!(
            "I'm a toast event! {:?}",
            Instant::now()
        )));
    }
}
