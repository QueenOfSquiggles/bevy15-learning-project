use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_tween::{
    combinator::{event, forward, sequence, tween},
    prelude::*,
    tween::AnimationTarget,
    tween_event::TweenEventPlugin,
};

use crate::settings::GameSettings;

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweenEventPlugin::<ToastTweenEvent>::default());
        app.add_systems(Startup, setup_toast);
        app.add_systems(Update, (test_toasts, toast_events));
        app.add_observer(register_new_toasts);
        app.add_observer(toast_container_despawn);
    }
}

#[derive(Event)]
pub struct ToastEvent(pub String);

#[derive(Component)]
struct ToastRoot;

#[derive(Component)]
struct ToastItem;

#[derive(Event)]
struct ToastItemDespawn;

#[derive(Clone, PartialEq, Eq, Default, Debug)]
enum ToastTweenEvent {
    #[default]
    None,
    Pop,
    End(Entity),
}

fn setup_toast(mut cmd: Commands) {
    cmd.spawn((
        ToastRoot,
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            align_content: AlignContent::Center,
            row_gap: Val::Px(10.0),
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
        const WIDTH: f32 = 320.0;
        let start = Vec3::X * -WIDTH;
        let end = Vec3::ZERO;
        let mut translate_tween = target.transform_state(Transform::from_translation(start));
        let text = trigger.event().0.clone();

        cmd.spawn(Node {
            max_width: Val::Px(WIDTH),
            ..default()
        })
        .with_children(|cmd| {
            let mut toast = cmd.spawn((
                Name::new(format!("Toast '{text}'")),
                ToastItem,
                Text::new(text.clone()),
                TextFont {
                    font: assets.load(settings.font.regular.clone()),
                    font_size: 24.0,
                    ..default()
                },
                BackgroundColor(css::ANTIQUE_WHITE.with_alpha(0.3).into()),
                BorderColor(css::BLACK.into()),
                BorderRadius::all(Val::Percent(15.0)),
                Node {
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                AnimationTarget,
            ));
            let entity = toast.id();
            info!("Toast ({entity}):  '{text}'");

            toast.animation().insert(sequence((
                tween(
                    Duration::from_secs_f32(0.5),
                    EaseKind::CubicOut,
                    translate_tween.translation_to(end),
                ),
                event(ToastTweenEvent::Pop),
                forward(Duration::from_secs_f32(1.0)),
                tween(
                    Duration::from_secs_f32(0.5),
                    EaseKind::CubicIn,
                    translate_tween.translation_to(start),
                ),
                event(ToastTweenEvent::End(entity)),
            )));
        });
    });
}

fn toast_events(
    mut events: EventReader<TweenEvent<ToastTweenEvent>>,
    mut cmd: Commands,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    parents: Query<&Parent>,
) {
    for event in events.read() {
        match event.data {
            ToastTweenEvent::Pop => {
                audio.play(assets.load("kenney_audio/click_001.ogg"));
            }
            ToastTweenEvent::End(entity) => {
                if let Ok(parent) = parents.get(entity) {
                    cmd.entity(parent.get()).trigger(ToastItemDespawn);
                } else {
                    cmd.entity(entity).remove_parent();
                    cmd.entity(entity).despawn_recursive();
                }
            }
            ToastTweenEvent::None => (),
        }
    }
}
fn toast_container_despawn(trigger: Trigger<ToastItemDespawn>, mut cmd: Commands) {
    cmd.entity(trigger.entity()).despawn_recursive();
}

fn test_toasts(keyboard: Res<ButtonInput<KeyCode>>, mut cmd: Commands) {
    if keyboard.just_pressed(KeyCode::Enter) {
        cmd.trigger(ToastEvent("I'm a toast event!".to_string()));
    }
}
