use crate::simulator::MoveQueue;
use bevy::prelude::*;
use cubesim::{Move, MoveVariant};

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

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut move_queue: ResMut<MoveQueue>) {
    if keyboard_input.just_pressed(KeyCode::F) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::F(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::F(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::B) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::B(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::B(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::L) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::L(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::L(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::R(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::R(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::U) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::U(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::U(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        if keyboard_input.pressed(KeyCode::LShift) {
            move_queue.push_back(Move::D(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::D(MoveVariant::Standard));
        }
    }
}
