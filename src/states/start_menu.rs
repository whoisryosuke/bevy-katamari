use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::AppState;

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_menu_setup.in_schedule(OnEnter(AppState::StartMenu)))
            .add_system(start_menu_system.in_set(OnUpdate(AppState::StartMenu)))
            .add_system(start_menu_cleanup.in_schedule(OnExit(AppState::StartMenu)));
    }
}

pub fn start_menu_setup() {
    println!("Start Menu setup");
}

pub fn start_menu_system(mut contexts: EguiContexts) {
    egui::Window::new("Start Menu").show(contexts.ctx_mut(), |ui| {
        ui.heading("General");
        ui.label("Position");
        // ui.add(egui::DragValue::new(&mut debug_state.debug_position.x).speed(1.0));
    });
}

pub fn start_menu_cleanup() {}
