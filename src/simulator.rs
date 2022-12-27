use crate::GameState;
use bevy::prelude::*;
use cubesim::{prelude::*, GeoCube};
use float_eq::float_eq;
use std::{
    collections::VecDeque,
    f32::consts::{FRAC_PI_2, PI},
    ops::{Deref, DerefMut},
};

pub struct SimulatorPlugin;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentCube::new(3))
            .init_resource::<MoveQueue>()
            .add_system(rotate_control)
            .add_system(rotate_piece)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(cube_setup)
                    .with_system(game_ui),
            );
    }
}

const UP_COLOR: Color = Color::WHITE;
const RIGHT_COLOR: Color = Color::RED;
const FRONT_COLOR: Color = Color::GREEN;
const DOWN_COLOR: Color = Color::YELLOW;
const LEFT_COLOR: Color = Color::ORANGE;
const BACK_COLOR: Color = Color::BLUE;
const PIECE_SIZE: f32 = 1.0;

const ROTATE_SPEED: f32 = 1.0;

#[derive(Resource)]
pub struct CurrentCube {
    geo_cube: GeoCube,
    cube_size: usize,
}

impl CurrentCube {
    pub fn new(cube_size: usize) -> Self {
        let geo_cube = GeoCube::new(cube_size as CubeSize);
        Self {
            geo_cube,
            cube_size,
        }
    }
}

impl Deref for CurrentCube {
    type Target = GeoCube;

    fn deref(&self) -> &Self::Target {
        &self.geo_cube
    }
}

impl DerefMut for CurrentCube {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.geo_cube
    }
}

#[derive(Component)]
struct Piece;

fn cube_setup(
    mut commands: Commands,
    current_cube: Res<CurrentCube>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("state {:?}", current_cube.state());

    let border = (current_cube.cube_size as f32 * PIECE_SIZE) / 2.0 - 0.5 * PIECE_SIZE;

    for (i, faces) in current_cube
        .state()
        .chunks(current_cube.cube_size * current_cube.cube_size)
        .collect::<Vec<&[Face]>>()
        .iter()
        .enumerate()
    {
        let saw_face = match i {
            0 => Face::U,
            1 => Face::R,
            2 => Face::F,
            3 => Face::D,
            4 => Face::L,
            5 => Face::B,
            _ => panic!("invalid index"),
        };

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
                let mut transform =
                    Transform::from_xyz(k as f32 - border, border, j as f32 - border);
                match saw_face {
                    Face::U => {}
                    Face::L => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(FRAC_PI_2));
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(FRAC_PI_2));
                    }
                    Face::F => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(FRAC_PI_2));
                    }
                    Face::R => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(-FRAC_PI_2));
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(FRAC_PI_2));
                    }
                    Face::B => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-FRAC_PI_2));
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(PI));
                    }
                    Face::D => {
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(PI));
                        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(PI));
                    }
                    Face::X => {
                        unreachable!()
                    }
                }

                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: PIECE_SIZE })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::BLACK,
                            unlit: true,
                            ..Default::default()
                        }),
                        transform,
                        ..Default::default()
                    })
                    .insert(Piece)
                    .with_children(|parent| {
                        parent.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Plane {
                                size: PIECE_SIZE * 0.9,
                            })),
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

#[derive(Resource, Default)]
pub struct MoveQueue {
    moves: VecDeque<Move>,
    rotating: bool,
}

impl Deref for MoveQueue {
    type Target = VecDeque<Move>;

    fn deref(&self) -> &Self::Target {
        &self.moves
    }
}

impl DerefMut for MoveQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.moves
    }
}

#[derive(Component)]
struct Rotating {
    axis: Vec3,
    angle: f32,
}

fn rotate_control(
    mut commands: Commands,
    mut move_queue: ResMut<MoveQueue>,
    current_cube: ResMut<CurrentCube>,
    q_not_rotating: Query<(Entity, &GlobalTransform), (Without<Rotating>, With<Piece>)>,
) {
    if move_queue.rotating {
        return;
    }
    if let Some(move_) = move_queue.pop_front() {
        current_cube.apply_move(move_);
        move_queue.rotating = true;
        let border = (current_cube.cube_size as f32 * PIECE_SIZE) / 2.0 - 0.5 * PIECE_SIZE;
        match move_ {
            Move::U(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().y, border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::L(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().x, -border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::F(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().z, border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::R(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().x, border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::B(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().z, -border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::D(v) => {
                for (entity, transform) in q_not_rotating.iter() {
                    if float_eq!(transform.translation().y, -border, abs <= 0.001) {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                    }
                }
            }
            Move::Uw(_, _) => {}
            Move::Lw(_, _) => {}
            Move::Fw(_, _) => {}
            Move::Rw(_, _) => {}
            Move::Bw(_, _) => {}
            Move::Dw(_, _) => {}
            Move::X(_) => {}
            Move::Y(_) => {}
            Move::Z(_) => {}
        }
    }
}

fn rotate_piece(
    mut commands: Commands,
    mut move_queue: ResMut<MoveQueue>,
    time: Res<Time>,
    mut q_rotating: Query<(Entity, &mut Transform, &mut Rotating), With<Piece>>,
) {
    if q_rotating.is_empty() {
        move_queue.rotating = false;
        return;
    }

    for (entity, mut transform, mut rotating) in q_rotating.iter_mut() {
        let mut rotate_angle = if rotating.angle > 0.0 {
            ROTATE_SPEED * PI * time.delta_seconds()
        } else {
            -ROTATE_SPEED * PI * time.delta_seconds()
        };

        rotating.angle -= rotate_angle;

        if rotate_angle > 0.0 && rotating.angle < 0.0 {
            rotate_angle = rotating.angle + rotate_angle;
            commands.entity(entity).remove::<Rotating>();
        } else if rotate_angle < 0.0 && rotating.angle > 0.0 {
            rotate_angle = rotating.angle + rotate_angle;
            commands.entity(entity).remove::<Rotating>();
        }

        let rotation = Quat::from_axis_angle(rotating.axis, rotate_angle);
        transform.rotate_around(Vec3::ZERO, rotation);
    }
}

#[derive(Component)]
struct GameUiRoot;

fn game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut move_queue: ResMut<MoveQueue>,
) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .insert(GameUiRoot)
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn(
                                TextBundle::from_section(
                                    "Back to menu",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                        });
                }); // root node
        });
}
