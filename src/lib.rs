use crate::{menu::MenuPlugin, player::PlayerPlugin, simulator::SimulatorPlugin};
use bevy::prelude::*;

mod menu;
mod player;
mod simulator;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
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
            .add_plugin(SimulatorPlugin)
            .add_plugin(MenuPlugin);

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
            //     .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
