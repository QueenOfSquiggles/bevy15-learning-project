use std::fs::File;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "settings.json";

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
    pub font: FontTypes,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct FontTypes {
    pub regular: String,
    pub bold: String,
    pub italic: String,
    pub bold_italic: String,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            font: FontTypes {
                regular: "font/noto_sans/regular.ttf".to_owned(),
                bold: "font/noto_sans/bold.ttf".to_owned(),
                italic: "font/noto_sans/italic.ttf".to_owned(),
                bold_italic: "font/noto_sans/regular.ttf".to_owned(),
            },
        }
    }
}

#[derive(Event)]
pub struct SettingsChanged;

fn load_settings(mut settings: ResMut<GameSettings>, mut cmd: Commands) {
    if let Ok(file) = File::open(SETTINGS_FILE) {
        if let Ok(new_settings) = serde_json::from_reader(file) {
            info!("Loaded settings from disk");
            *settings = new_settings;
        } else {
            warn!("Malformed settings on disk, this will be overriden by the default values");
        }
    } else {
        info!("Failed to open settings file, this is usually just fine, unless that file is expected to be on disk");
    }
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
