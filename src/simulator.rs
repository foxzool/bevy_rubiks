use crate::GameState;
use bevy::prelude::*;
use cubesim::{prelude::*, GeoCube};
use std::f32::consts::{FRAC_PI_2, PI};

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentCube::new(3))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(cube_setup));
    }
}

const UP_COLOR: Color = Color::WHITE;
const RIGHT_COLOR: Color = Color::RED;
const FRONT_COLOR: Color = Color::GREEN;
const DOWN_COLOR: Color = Color::YELLOW;
const LEFT_COLOR: Color = Color::ORANGE;
const BACK_COLOR: Color = Color::BLUE;

#[derive(Resource)]
struct CurrentCube {
    geo_cube: GeoCube,
    cube_size: usize,
}

impl CurrentCube {
    fn new(cube_size: usize) -> Self {
        let geo_cube = GeoCube::new(cube_size as CubeSize);
        Self {
            geo_cube,
            cube_size,
        }
    }

    fn state(&self) -> Vec<Face> {
        self.geo_cube.state()
    }
}

fn cube_setup(
    mut commands: Commands,
    mut current_cube: ResMut<CurrentCube>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("{:?}", current_cube.state());

    for faces in current_cube
        .state()
        .chunks(current_cube.cube_size * current_cube.cube_size)
        .collect::<Vec<&[Face]>>()
        .iter()
    {
        for (j, faces) in faces.chunks(current_cube.cube_size).enumerate() {
            for (k, face) in faces.iter().enumerate() {
                let color = match face {
                    Face::U => UP_COLOR,
                    Face::L => LEFT_COLOR,
                    Face::F => FRONT_COLOR,
                    Face::R => RIGHT_COLOR,
                    Face::B => BACK_COLOR,
                    Face::D => DOWN_COLOR,
                    Face::X => {
                        unreachable!()
                    }
                };
                let mut transform = Transform::from_xyz(k as f32 - 1.0, 1.0, j as f32 - 1.0);
                match face {
                    Face::U => {}
                    Face::L => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(FRAC_PI_2));
                    }
                    Face::F => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(FRAC_PI_2));
                    }
                    Face::R => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(-FRAC_PI_2));
                    }
                    Face::B => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-FRAC_PI_2));
                    }
                    Face::D => transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(PI)),
                    Face::X => {
                        unreachable!()
                    }
                }

                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::BLACK,
                            unlit: true,
                            ..Default::default()
                        }),
                        transform,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.9 })),
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                unlit: true,
                                ..Default::default()
                            }),
                            transform: Transform::from_xyz(0.0, 0.501, 0.0),
                            ..Default::default()
                        });
                    });
            }
        }
    }
}
