use std::{cmp, f32::consts::PI};
mod player;

use bevy::{
    gizmos,
    math::{bounding::RayCast2d, vec2},
    prelude::*,
    render::color,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform,
};

#[derive(Component)]
struct Wall {
    start: Vec2,
    end: Vec2,
}

#[derive(Component, Debug)]
struct Player {
    ray: RayCast2d,
}

#[derive(Component, Debug)]
enum Shape {
    Line,
}

#[derive(Component, Debug)]
struct Map {
    map: [[i32; 10]; 10],
}

fn add_walls(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.width() / 2.;
    let height = window.height() / 2.;
    commands.spawn((Wall {
        start: Vec2::from_array([0.0, -height]),
        end: Vec2::from_array([0.0, height]),
    },));
    commands.spawn((Wall {
        start: Vec2::from_array([-width, -height / 3.5]),
        end: Vec2::from_array([width, -height / 3.5]),
    },));
}

fn get_intersection(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Option<Vec2> {
    // Calculate the intersection point of two lines
    let a1 = b.y - a.y;
    let b1 = a.x - b.x;
    let c1 = a1 * a.x + b1 * a.y;

    let a2 = d.y - c.y;
    let b2 = c.x - d.x;
    let c2 = a2 * c.x + b2 * c.y;

    let det = a1 * b2 - a2 * b1;
    if det == 0. {
        println!("Lines are parallel");
        return None;
    } else {
        return Some(Vec2::new(
            (b2 * c1 - b1 * c2) / det,
            (a1 * c2 - a2 * c1) / det,
        ));
    }
}

fn ccw(a: Vec2, b: Vec2, c: Vec2) -> bool {
    return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x);
}

fn intersects(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    return ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d);
}

fn get_intersection_point(start: Vec2, end: Vec2, wall: &Wall) -> Option<Vec2> {
    if (intersects(start, end, wall.start, wall.end)) {
        return get_intersection(start, end, wall.start, wall.end);
    }
    None
}

fn draw_walls(mut gizmos: Gizmos, query: Query<&Wall>) {
    for wall in &query {
        gizmos.line_2d(wall.start, wall.end, Color::WHITE);
    }
}

fn render_shapes(
    mut gizmos: Gizmos,
    mut query: Query<(&Shape, &Transform)>,
    wall_query: Query<&Wall>,
) {
    for (shape, transform) in query.iter_mut() {
        let mut intersections: Vec<Vec2> = Vec::new();
        let translation = transform.translation.xy();
        let rotation = Vec2::new(transform.rotation.z, transform.rotation.w);
        let mut dist = 150.;
        let mut end = translation + (rotation * dist);

        for wall in &wall_query {
            if let Some(intersection) = get_intersection_point(translation, end, wall) {
                intersections.push(intersection);
            }
        }
        if !intersections.is_empty() {
            let closest = intersections
                .iter()
                .min_by(|a, b| {
                    a.distance(translation)
                        .partial_cmp(&b.distance(translation))
                        .unwrap()
                })
                .unwrap();
            dist = translation.distance(*closest);
            end = translation + (rotation * dist);
        }

        match shape {
            Shape::Line => {
                gizmos.line_2d(translation, end, Color::WHITE);
                gizmos.circle_2d(end, 10., Color::BLUE);
            }
        }
    }
}

fn general_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        (Map {
            map: [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 1, 0, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 1, 0, 1, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 0, 0, 1],
                [1, 0, 0, 1, 0, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        }),
    );
}

fn raycaster_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_query: Query<&Map>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(10., 10., 0.),
            ..default()
        },
        Player {
            ray: RayCast2d::from_ray(
                Ray2d {
                    origin: Vec2::from_array([0.5, 0.5]).normalize(),
                    direction: Direction2d::new_unchecked(Vec2::new(0.1, 0.1).normalize()),
                },
                150.0,
            ),
        },
        Shape::Line,
    ));
    let map = map_query.single().map;
    let window = window.single();
    let tile_height = window.height() / 10.;
    let tile_width = window.width() / 10.;
    for i in 0..10 {
        for j in 0..10 {
            if map[i][j] == 1 {
                commands.spawn((SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (i as f32 - 5.) * tile_width + tile_width / 2.,
                            (j as f32 - 5.) * tile_height + tile_height / 2.,
                            0.,
                        ),
                        scale: Vec3::new(tile_width, tile_height, 1.0),
                        ..default()
                    },
                    ..default()
                },));
            }
        }
    }
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::PURPLE),
        ..default()
    });
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let direction = Vec3::new(transform.rotation.z, transform.rotation.w, 0.);
            let distance = time.delta_seconds() * 100.;
            transform.translation += distance * direction;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let direction = Vec3::new(transform.rotation.z, transform.rotation.w, 0.);
            let distance = time.delta_seconds() * 100.;
            transform.translation -= distance * direction;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotation *= Quat::from_rotation_z(-time.delta_seconds() * 5.);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * 5.);
        }
    }
}

pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (general_setup, raycaster_setup, add_walls).chain())
            .add_systems(PostUpdate, (draw_walls, render_shapes).chain())
            .add_systems(FixedUpdate, move_player);
    }
}

pub struct FpsRayCastPlugin;

impl Plugin for FpsRayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (general_setup, setup_fps_ray_cast).chain())
            .add_systems(Update, (ray_cast, move_ray_cast_player).chain());
    }
}

fn move_ray_cast_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut map_query: Query<&mut Map>,
    time: Res<Time>,
) {
    let map = map_query.single_mut();
    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let direction = Vec3::new(transform.rotation.z, transform.rotation.w, 0.);
            let distance = time.delta_seconds() * 10.;

            let new_direction = transform.translation - distance * direction;
            if map.map[new_direction.y.floor() as usize][new_direction.x.floor() as usize] == 0 {
                transform.translation = new_direction;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let direction = Vec3::new(transform.rotation.z, transform.rotation.w, 0.);
            let distance = time.delta_seconds() * 10.;
            let new_direction = transform.translation + distance * direction;
            if map.map[new_direction.y.floor() as usize][new_direction.x.floor() as usize] == 0 {
                transform.translation = new_direction;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotation *= Quat::from_rotation_z(-time.delta_seconds() * 5.);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * 5.);
        }
    }
}

fn cast_ray(ray_angle: f32, original_x: f32, original_y: f32, map: &Map) -> (f32, f32) {
    let mut x = original_x;
    let mut y = original_y;
    let mut dx = ray_angle.cos();
    let mut dy = ray_angle.sin();

    let mut i = 0;
    while (map.map[y.floor() as usize][x.floor() as usize] == 0) {
        x += (dx * 0.1);
        y += (dy * 0.1);
        i += 1;
        if i > 100 {
            break;
        }
    }

    let distance = ((x - original_x).powf(2.0) + (y - original_y).powf(2.0)).sqrt();
    let wall_height = 300.0 / distance;
    return (distance, wall_height);
}

fn draw_wall_slice(
    i: f32,
    wall_height: f32,
    slice_width: f32,
    dither_pattern_size: f32,
    distance: f32,
    gizmos: &mut Gizmos,
) {
    let darkness_factor = 1. + (distance / 4.);

    for j in 0..wall_height as i32 {
        let y_position = -300. + (300. - wall_height / 2. + j as f32).floor();
        let dither = if ((i + y_position) % dither_pattern_size) < (dither_pattern_size / 2.) {
            10.
        } else {
            0.
        };

        let base_color = 180. + dither;
        let adjusted_color = (base_color / darkness_factor).floor() as f32;
        let color = Color::rgb_u8(adjusted_color as u8, 0, adjusted_color as u8);

        gizmos.rect_2d(
            Vec2::new(i, y_position),
            0.,
            //Vec2::ONE * slice_width,
            Vec2::ONE * Vec2::new(slice_width, wall_height),
            color,
        );
    }
}

fn ray_cast(
    window: Query<&Window>,
    query: Query<&Transform, With<Player>>,
    map_query: Query<&Map>,
    mut gizmos: Gizmos,
) {
    let window = window.single();
    let rays = 200;
    let dither_pattern_size = 8.;
    let screen_width = window.width();
    let screen_height = window.height();
    let y_index = -screen_width / 2.;
    let x_index = -screen_height / 2.;
    let slice_width = screen_width as f32 / rays as f32;
    let player_fov = PI / 4.;
    let angle_step = player_fov / rays as f32;
    let map = map_query.single();
    gizmos.line_2d(
        Vec2::new(-screen_width / 2., -screen_height / 2.),
        Vec2::new(screen_width / 2., screen_height / 2.),
        Color::RED,
    );
    for transform in &query {
        let player_angle = transform.rotation.to_axis_angle().1;

        for i in 0..rays {
            //let angle = angle_step * i as f32;
            let angle = player_angle - (player_fov / 2.) + (i as f32 * angle_step);

            let (distance, wall_height) = cast_ray(
                angle.to_radians(),
                transform.translation.x,
                transform.translation.y,
                map,
            );
            draw_wall_slice(
                (-100. + i as f32) * slice_width,
                wall_height,
                slice_width,
                dither_pattern_size,
                distance,
                &mut gizmos,
            );
        }
    }
}

fn setup_fps_ray_cast(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width = window.width();
    let height = window.height();
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(3., 3., 1.),
            ..default()
        },
        Player {
            ray: RayCast2d::from_ray(
                Ray2d {
                    origin: Vec2::from_array([0.5, 0.5]).normalize(),
                    direction: Direction2d::new_unchecked(Vec2::new(0.1, 0.1).normalize()),
                },
                150.0,
            ),
        },
        Shape::Line,
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(width, height)).into(),
        transform: Transform::from_xyz(0., height / 2., 0.),
        material: materials.add(Color::rgb_u8(20, 0, 20)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(width, height / 2.)).into(),
        transform: Transform::from_xyz(0., -height / 4., 0.),
        material: materials.add(Color::rgb_u8(60, 0, 60)),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RayCastPlugin))
        .run();
}
