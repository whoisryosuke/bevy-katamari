use bevy::prelude::*;

use self::{game::GamePlugin, start_menu::StartMenuPlugin};

mod game;
mod start_menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    StartMenu,
    Game,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(GamePlugin)
            .add_plugin(StartMenuPlugin);
    }
}
