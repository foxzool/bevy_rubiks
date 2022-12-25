use crate::simulator::CurrentCube;
use bevy::prelude::*;
use cubesim::{Cube, Move, MoveVariant};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(keyboard_input_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.5, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, current_cube: ResMut<CurrentCube>) {
    if keyboard_input.just_pressed(KeyCode::F) {
        current_cube.apply_move(Move::F(MoveVariant::Standard));
    }
}
