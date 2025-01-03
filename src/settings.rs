use std::fs::File;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &'static str = "settings.json";

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
        app.add_systems(Startup, load_settings);
        app.add_observer(observe_settings_changed);
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub font: String,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            font: "font/noto_sans/regular.ttf".to_owned(),
        }
    }
}

#[derive(Event)]
pub struct SettingsChanged;

fn load_settings(mut settings: ResMut<GameSettings>, mut cmd: Commands) {
    let Ok(file) = File::open(SETTINGS_FILE) else {
        return;
    };
    let Ok(new_settings) = serde_json::from_reader(file) else {
        return;
    };
    *settings = new_settings;
    cmd.trigger(SettingsChanged);
}

fn observe_settings_changed(_: Trigger<SettingsChanged>, settings: Res<GameSettings>) {
    let Ok(file) = File::create(SETTINGS_FILE) else {
        return;
    };
    if let Err(e) = serde_json::to_writer_pretty(file, settings.into_inner()) {
        warn!("Failed to serialize game settings: {e}");
    }
}
