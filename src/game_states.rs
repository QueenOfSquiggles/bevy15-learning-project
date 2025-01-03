use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<PauseState>()
            .init_state::<MouseState>()
            .add_systems(Update, toggle_paused)
            .add_systems(OnEnter(PauseState::Running), enter_running_state)
            .add_systems(OnEnter(PauseState::Paused), enter_pause_state)
            .add_systems(OnEnter(MouseState::Captured), enter_mouse_captured)
            .add_systems(OnEnter(MouseState::Free), enter_mouse_free);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum MouseState {
    #[default]
    Captured,
    Free,
}

fn toggle_paused(
    keyboard: Res<ButtonInput<KeyCode>>,
    current: Res<State<PauseState>>,
    mut next: ResMut<NextState<PauseState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next.set(match current.get() {
            PauseState::Running => PauseState::Paused,
            PauseState::Paused => PauseState::Running,
        });
    }
}

fn enter_running_state(mut next_mouse_state: ResMut<NextState<MouseState>>) {
    next_mouse_state.set(MouseState::Captured);
}

fn enter_pause_state(mut next_mouse_state: ResMut<NextState<MouseState>>) {
    next_mouse_state.set(MouseState::Free);
}

fn enter_mouse_captured(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = q_window.get_single_mut() else {
        return;
    };
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}
fn enter_mouse_free(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = q_window.get_single_mut() else {
        return;
    };
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
}
