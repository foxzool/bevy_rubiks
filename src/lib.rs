use crate::{menu::MenuPlugin, player::PlayerPlugin, simulator::SimulatorPlugin};
use bevy::prelude::*;

mod menu;
mod player;
mod simulator;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
#[allow(dead_code)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Solved,
}

pub struct RubiksPlugin;

impl Plugin for RubiksPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(SimulatorPlugin)
            .add_plugins(MenuPlugin);

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
            //     .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
