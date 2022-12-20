use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod menu;
mod player;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Playing,
    Solved,
}

pub struct RubiksPlugin;

impl Plugin for RubiksPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Menu)
            .add_plugin(PlayerPlugin)
            .add_plugin(MenuPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
