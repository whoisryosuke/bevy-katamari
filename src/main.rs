use bevy::{prelude::*, window::*};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use debug::DebugPlugin;
use states::AppStatePlugin;

mod debug;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1024., 768.),
                title: "Bevy Jam 420".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DebugPlugin)
        .add_plugin(AppStatePlugin)
        .run();
}
