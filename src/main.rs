use std::fs::File;

use avian3d::prelude::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy::{
    input::common_conditions::input_toggle_active,
    log::{tracing_subscriber::fmt::Layer, BoxedLayer, LogPlugin},
    prelude::*,
};
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_tween::DefaultTweenPlugins;
use game_states::GameStatesPlugin;
use health::HealthPlugin;
use items::ItemsPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use post_process::PostProcessPlugin;
use seldom_state::StateMachinePlugin;
use settings::SettingsPlugin;
use stats::RpgStatsPlugin;
use toast::ToastPlugin;

mod game_states;
mod health;
mod items;
mod level;
mod player;
mod post_process;
mod settings;
mod stats;
mod toast;

fn main() {
    App::new()
        // Note: bevy tuple collections only work up to 20 entries due to rust shenanigans, I just use more nested tuples but there are other options (Making "Macro Plugins" for different plugin groups)
        .add_plugins((
            // bevy built-in plugins
            DefaultPlugins.set(LogPlugin {
                custom_layer: setup_custom_logging_step,
                ..default()
            }),
            (
                // Third Party plugins
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backspace)),
                PhysicsPlugins::default().with_length_unit(1.0),
                PhysicsDebugPlugin::default(),
                TnuaControllerPlugin::new(FixedUpdate),
                TnuaAvian3dPlugin::new(FixedUpdate),
                DefaultTweenPlugins,
                AudioPlugin,
                HanabiPlugin,
                StateMachinePlugin,
            ),
            (
                // my Plugins
                PlayerPlugin,
                GameStatesPlugin,
                ItemsPlugin,
                ToastPlugin,
                SettingsPlugin,
                LevelPlugin,
                RpgStatsPlugin,
                HealthPlugin,
                PostProcessPlugin,
            ),
        ))
        .add_systems(Update, quit_on_f8)
        .run();
}

fn setup_custom_logging_step(_: &mut App) -> Option<BoxedLayer> {
    let Ok(file) = File::create("app.log") else {
        return None;
    };
    let layer = Layer::new()
        .with_writer(file)
        .with_level(true)
        .with_ansi(false);
    Some(Box::new(layer))
}

fn quit_on_f8(buttons: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if buttons.any_just_pressed([KeyCode::F8]) {
        writer.send(AppExit::Success);
    }
}
