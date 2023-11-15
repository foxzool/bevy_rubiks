use crate::simulator::MoveQueue;
use bevy::prelude::*;
use cubesim::{Move, MoveVariant};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, keyboard_input_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.5, 3.5, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut move_queue: ResMut<MoveQueue>) {
    let move_variant = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        MoveVariant::Inverse
    } else if keyboard_input.pressed(KeyCode::Key2) {
        MoveVariant::Double
    } else {
        MoveVariant::Standard
    };

    if keyboard_input.just_pressed(KeyCode::F) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Fw(2, move_variant));
        } else {
            move_queue.push_back(Move::F(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::B) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Bw(2, move_variant));
        } else {
            move_queue.push_back(Move::B(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::L) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Lw(2, move_variant));
        } else {
            move_queue.push_back(Move::L(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Rw(2, move_variant));
        } else {
            move_queue.push_back(Move::R(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::U) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Uw(2, move_variant));
        } else {
            move_queue.push_back(Move::U(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        if keyboard_input.pressed(KeyCode::ControlLeft) {
            move_queue.push_back(Move::Dw(2, move_variant));
        } else {
            move_queue.push_back(Move::D(move_variant));
        }
    }

    if keyboard_input.just_pressed(KeyCode::X) {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            move_queue.push_back(Move::X(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::X(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::Y) {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            move_queue.push_back(Move::Y(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::Y(MoveVariant::Standard));
        }
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            move_queue.push_back(Move::Z(MoveVariant::Inverse));
        } else {
            move_queue.push_back(Move::Z(MoveVariant::Standard));
        }
    }
}
