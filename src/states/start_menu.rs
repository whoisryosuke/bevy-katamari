use bevy::prelude::*;

use super::AppState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_menu_setup.in_schedule(OnEnter(AppState::StartMenu)))
            .add_system(start_menu_system.in_set(OnUpdate(AppState::StartMenu)))
            .add_system(start_menu_cleanup.in_schedule(OnExit(AppState::StartMenu)));
    }
}

pub fn start_menu_setup() {}

pub fn start_menu_system() {}

pub fn start_menu_cleanup() {}
