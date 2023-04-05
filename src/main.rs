use bevy::{prelude::*, window::*};
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
        .add_plugin(DebugPlugin)
        .add_plugin(AppStatePlugin)
        .run();
}
