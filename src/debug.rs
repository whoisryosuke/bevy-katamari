use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_rapier3d::prelude::*;

// App-level debug state
#[derive(Resource)]
struct DebugState {
    // Is debug menu visible?
    visible: bool,
    // A general position value to play with
    debug_position: Vec3,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugState {
            visible: false,
            debug_position: Vec3::splat(0.0),
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(debug_ui)
        .add_system(debug_controls);
    }
}

fn debug_ui(mut contexts: EguiContexts, mut debug_state: ResMut<DebugState>) {
    if debug_state.visible {
        egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
            ui.heading("General");
            ui.label("Position");
            ui.add(egui::DragValue::new(&mut debug_state.debug_position.x).speed(1.0));
        });
    }
}

fn debug_controls(keyboard_input: Res<Input<KeyCode>>, mut debug_state: ResMut<DebugState>) {
    if keyboard_input.pressed(KeyCode::LShift) && keyboard_input.just_released(KeyCode::P) {
        if debug_state.visible {
            debug_state.visible = false;
        } else {
            debug_state.visible = true;
        }
    }
}
