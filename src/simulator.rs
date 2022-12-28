use crate::GameState;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use cubesim::{prelude::*, random_scramble, solve, FaceletCube, GeoCube};
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
            .add_system(button_system)
            .add_system(mouse_scroll)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(cube_setup)
                    .with_system(game_ui),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clean_up));
    }
}

const UP_COLOR: Color = Color::WHITE;
const RIGHT_COLOR: Color = Color::RED;
const FRONT_COLOR: Color = Color::GREEN;
const DOWN_COLOR: Color = Color::YELLOW;
const LEFT_COLOR: Color = Color::ORANGE;
const BACK_COLOR: Color = Color::BLUE;
const PIECE_SIZE: f32 = 1.0;

const ROTATE_SPEED: f32 = 2.0;

#[derive(Resource)]
pub struct CurrentCube {
    geo_cube: GeoCube,
    cube_size: usize,
    moves: Vec<Move>,
}

impl CurrentCube {
    pub fn new(cube_size: usize) -> Self {
        let geo_cube = GeoCube::new(cube_size as CubeSize);
        Self {
            geo_cube,
            cube_size,
            moves: vec![],
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
    mut current_cube: ResMut<CurrentCube>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    current_cube.geo_cube = GeoCube::new(current_cube.cube_size as CubeSize);
    current_cube.moves = vec![];
    let border = (current_cube.cube_size as f32 * PIECE_SIZE) / 2.0 - 0.5 * PIECE_SIZE;
    info!("state {:?}", current_cube.state());
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

type NotRotatingPiece = (Without<Rotating>, With<Piece>);

fn rotate_control(
    mut commands: Commands,
    mut move_queue: ResMut<MoveQueue>,
    mut current_cube: ResMut<CurrentCube>,
    q_not_rotating: Query<(Entity, &GlobalTransform), NotRotatingPiece>,
    q_rotating: Query<&Rotating>,
    mut q_text: Query<&mut Text, With<MovesText>>,
) {
    if !q_rotating.is_empty() {
        return;
    }
    if let Some(move_) = move_queue.pop_front() {
        current_cube.geo_cube = current_cube.apply_move(move_);
        current_cube.moves.push(move_);
        let mut text = q_text.single_mut();

        text.sections[0].value = current_cube
            .moves
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        debug!("move {}", move_);
        let border = (current_cube.cube_size as f32 * PIECE_SIZE) / 2.0 - 0.5 * PIECE_SIZE;
        match move_ {
            Move::U(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().y >= border - 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });

                        count += 1;
                    }
                }

                trace!("U apply {count}");
            }
            Move::L(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().x <= -border + 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("L apply {count}");
            }
            Move::F(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().z >= border - 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                debug!("F apply {count}");
            }
            Move::R(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().x >= border - 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("R apply {count}");
            }
            Move::B(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().z <= -border + 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }
                trace!("B apply {count}");
            }
            Move::D(v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if transform.translation().y <= -border + 0.01 {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                debug!("D apply {count}");
            }
            Move::Uw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if (((border - slice as f32 / 2.0) - 0.001)..=(border + 0.001))
                        .contains(&transform.translation().y)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });

                        count += 1;
                    }
                }

                trace!("Uw apply {count}");
            }
            Move::Lw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if ((-border - 0.001)..=(border - (slice as f32 / 2.0) + 0.001))
                        .contains(&transform.translation().x)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("Lw apply {count}");
            }
            Move::Fw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if (((border - slice as f32 / 2.0) - 0.001)..=(border + 0.001))
                        .contains(&transform.translation().z)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("Fw apply {count}");
            }
            Move::Rw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if (((border - slice as f32 / 2.0) - 0.001)..=(border + 0.001))
                        .contains(&transform.translation().x)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::X,
                            angle: match v {
                                MoveVariant::Standard => -FRAC_PI_2,
                                MoveVariant::Inverse => FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });

                        count += 1;
                    }
                }

                trace!("Rw apply {count}");
            }
            Move::Bw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if ((-border - 0.001)..=((border - slice as f32 / 2.0) + 0.001))
                        .contains(&transform.translation().z)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Z,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("Bw apply {count}");
            }
            Move::Dw(slice, v) => {
                let mut count = 0;
                for (entity, transform) in q_not_rotating.iter() {
                    if ((-border - 0.001)..=((border - slice as f32 / 2.0) + 0.001))
                        .contains(&transform.translation().y)
                    {
                        commands.entity(entity).insert(Rotating {
                            axis: Vec3::Y,
                            angle: match v {
                                MoveVariant::Standard => FRAC_PI_2,
                                MoveVariant::Inverse => -FRAC_PI_2,
                                MoveVariant::Double => PI,
                            },
                        });
                        count += 1;
                    }
                }

                trace!("Dw apply {count}");
            }
            Move::X(v) => {
                for (entity, _transform) in q_not_rotating.iter() {
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
            Move::Y(v) => {
                for (entity, _transform) in q_not_rotating.iter() {
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
            Move::Z(v) => {
                for (entity, _transform) in q_not_rotating.iter() {
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
    }
}

fn rotate_piece(
    mut commands: Commands,
    time: Res<Time>,
    mut q_rotating: Query<(Entity, &mut Transform, &mut Rotating), With<Piece>>,
) {
    for (entity, mut transform, mut rotating) in q_rotating.iter_mut() {
        let mut rotate_angle = if rotating.angle > 0.0 {
            ROTATE_SPEED * PI * time.delta_seconds()
        } else {
            -ROTATE_SPEED * PI * time.delta_seconds()
        };

        rotating.angle -= rotate_angle;

        if (rotate_angle > 0.0 && rotating.angle < 0.0)
            || (rotate_angle < 0.0 && rotating.angle > 0.0)
        {
            rotate_angle += rotating.angle;
            commands.entity(entity).remove::<Rotating>();
        }

        let rotation = Quat::from_axis_angle(rotating.axis, rotate_angle);
        transform.rotate_around(Vec3::ZERO, rotation);
    }
}

#[derive(Component)]
struct GameUiRoot;

#[derive(Component)]
enum PlayButtonActions {
    BackToMenu,
    CubeScramble,
    CubeSolver,
}

#[derive(Component)]
struct MovesText;

fn game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "Back to menu",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    }),
                                )
                                .insert(PlayButtonActions::BackToMenu)
                                .insert(Interaction::None);

                            // text
                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "Cube Scramble",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    }),
                                )
                                .insert(PlayButtonActions::CubeScramble)
                                .insert(Interaction::None);

                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "Apply solver",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(15.0)),
                                        ..default()
                                    }),
                                )
                                .insert(PlayButtonActions::CubeSolver)
                                .insert(Interaction::None);
                        });
                }); // root node

            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Moves",
                            TextStyle {
                                font: font.clone(),
                                font_size: 35.,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_alignment(TextAlignment::CENTER)
                        .with_style(Style {
                            size: Size::new(Val::Undefined, Val::Px(25.)),
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..default()
                            },
                            ..default()
                        }),
                    );

                    parent
                        .spawn(
                            TextBundle::from_section(
                                String::new(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_alignment(TextAlignment::CENTER)
                            .with_style(Style {
                                position: UiRect {
                                    top: Val::Px(5.0),
                                    left: Val::Px(5.0),
                                    ..default()
                                },
                                max_size: Size {
                                    width: Val::Px(180.),
                                    height: Val::Undefined,
                                },
                                ..default()
                            }),
                        )
                        .insert(MovesText);
                });
        });
}

fn clean_up(
    mut commands: Commands,
    q_ui: Query<Entity, With<GameUiRoot>>,
    q_piece: Query<Entity, With<Piece>>,
    mut move_queue: ResMut<MoveQueue>,
) {
    move_queue.moves.clear();
    for entity in q_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in q_piece.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &PlayButtonActions), (Changed<Interaction>,)>,
    mut game_state: ResMut<State<GameState>>,
    current_cube: Res<CurrentCube>,
    mut move_queue: ResMut<MoveQueue>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            match *button {
                PlayButtonActions::BackToMenu => {
                    game_state.set(GameState::Menu).unwrap();
                }
                PlayButtonActions::CubeScramble => {
                    let mut cmds: VecDeque<Move> =
                        random_scramble(current_cube.cube_size as CubeSize, false).into();

                    move_queue.moves.append(&mut cmds);
                }
                PlayButtonActions::CubeSolver => {
                    let cube = FaceletCube::new(current_cube.cube_size as CubeSize)
                        .apply_moves(&current_cube.moves);
                    let solution = solve(&cube);

                    if let Some(s) = solution {
                        let mut solution = String::new();
                        for m in s.iter() {
                            solution.push_str(&m.to_string());
                            solution.push(' ');
                            move_queue.moves.push_back(*m);
                        }
                        info!("Solution {}", solution);
                    } else {
                        warn!("Facelet Cube {:?} no solver", cube.state());
                    }
                }
            }
        }
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
