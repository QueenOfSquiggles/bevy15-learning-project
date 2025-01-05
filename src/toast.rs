use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_tween::{
    combinator::{event, forward, sequence, tween},
    prelude::*,
    tween::AnimationTarget,
};

use crate::settings::GameSettings;

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_toast);
        app.add_systems(Update, test_toasts);
        app.add_observer(register_new_toasts);
    }
}

#[derive(Event)]
pub struct ToastEvent(pub String);

#[derive(Component)]
struct ToastRoot;

#[derive(Component)]
struct ToastItem;

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

    cmd.entity(root).with_children(|cmd| {
        let target = AnimationTarget.into_target();
        let start = Vec3::X * -2000.0;
        let end = Vec3::ZERO;
        let mut translate_tween = target.transform_state(Transform::from_translation(start));
        let mut toast = cmd.spawn((
            ToastItem,
            Text::new(trigger.event().0.clone()),
            TextFont {
                font: assets.load(settings.font.clone()),
                font_size: 24.0,
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0.2, 0.2, 0.2, 0.4)),
            BorderColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            BorderRadius::all(Val::Percent(15.0)),
            Node {
                overflow: Overflow::clip(),
                ..default()
            },
            AnimationTarget,
        ));

        toast.animation().insert(sequence((
            tween(
                Duration::from_secs_f32(1.5),
                EaseKind::CubicOut,
                translate_tween.translation_to(end),
            ),
            event("pop"),
            forward(Duration::from_secs_f32(5.0)),
            tween(
                Duration::from_secs_f32(1.5),
                EaseKind::CubicIn,
                translate_tween.translation_to(start),
            ),
            event("end"),
        )));
        let toast_entity = toast.id();
        toast.observe(
            move |t: Trigger<TweenEvent<&str>>,
                  mut cmd: Commands,
                  audio: Res<Audio>,
                  assets: Res<AssetServer>| match t.data {
                "pop" => {
                    info!("Pop event");
                    audio.play(assets.load("kenney_audio/click_001.ogg"));
                }
                "end" => {
                    info!("End event");
                    cmd.entity(toast_entity).remove_parent();
                    cmd.entity(toast_entity).despawn_recursive();
                }
                _ => (),
            },
        );
    });
}

fn test_toasts(keyboard: Res<ButtonInput<KeyCode>>, mut cmd: Commands) {
    if keyboard.just_pressed(KeyCode::Enter) {
        cmd.trigger(ToastEvent(format!("I'm a toast event!")));
    }
}
