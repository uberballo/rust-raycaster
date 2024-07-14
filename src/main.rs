use std::cmp;
mod player;

use bevy::{
    math::{bounding::RayCast2d, vec2},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    ui::update,
};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Herra Huu".to_string())));
    commands.spawn((Person, Name("Wtf wtf".to_string())));
}

#[derive(Component)]
struct Wall {
    start: Vec2,
    end: Vec2,
}

fn add_walls(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.width() / 2.;
    let height = window.height() / 2.;
    commands.spawn((Wall {
        start: Vec2::from_array([0.5, -height]),
        end: Vec2::from_array([0.5, height]),
    },));
    println!("width: {}, height: {}", width, height);
    commands.spawn((Wall {
        start: Vec2::from_array([-width, -height / 3.5]),
        end: Vec2::from_array([width, -height / 3.5]),
    },));
}

#[derive(Component)]
struct Player {
    ray: RayCast2d,
}

#[derive(Resource)]
struct Game {
    player: Player,
}

fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        Color::WHITE,
    );
    for r in [1., 2., 3.] {
        gizmos.circle_2d(ray.ray.origin, r, Color::FUCHSIA);
    }
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

//fn draw_walls(mut gizmos: Gizmos, query: Query<&Wall>, time: Res<Time>) {
//    let ray = Vec2::from_array([0.5, 0.5]);
//    let direction = Vec2::new(time.elapsed_seconds().cos(), time.elapsed_seconds().sin());
//    let mut dist = 150.;
//    let mut intersections: Vec<Vec2> = Vec::new();
//    let aabb_ray = Ray2d {
//        origin: ray * 50.,
//        direction: Direction2d::new_unchecked(direction),
//    };
//    for wall in &query {
//        gizmos.line_2d(wall.start, wall.end, Color::WHITE);
//        if (intersects(
//            aabb_ray.origin,
//            aabb_ray.get_point(dist),
//            wall.start,
//            wall.end,
//        )) {
//            let intersection = get_intersection(
//                aabb_ray.origin,
//                aabb_ray.get_point(dist),
//                wall.start,
//                wall.end,
//            );
//            if let Some(intersection) = intersection {
//                gizmos.circle_2d(intersection, 10., Color::GREEN);
//                intersections.push(intersection);
//            }
//        }
//        gizmos.circle_2d(aabb_ray.get_point(dist), 10., Color::BLUE);
//    }
//
//    if !intersections.is_empty() {
//        let closest = intersections
//            .iter()
//            .min_by(|a, b| {
//                a.distance(aabb_ray.origin)
//                    .partial_cmp(&b.distance(aabb_ray.origin))
//                    .unwrap()
//            })
//            .unwrap();
//        dist = aabb_ray.origin.distance(*closest);
//    }
//
//    println!("dist: {}", dist);
//
//    let player = Player {
//        ray: RayCast2d::from_ray(aabb_ray, dist),
//    };
//
//    draw_ray(&mut gizmos, &player.ray);
//}

fn draw_walls(
    mut gizmos: Gizmos,
    query: Query<&Wall>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    //let ray = Vec2::from_array([0.5, 0.5]);
    //let direction = Vec2::new(time.elapsed_seconds().cos(), time.elapsed_seconds().sin());
    //let mut dist = 150.;
    let mut intersections: Vec<Vec2> = Vec::new();
    //let aabb_ray = Ray2d {
    //    origin: ray * 50.,
    //    direction: Direction2d::new_unchecked(direction),
    //};
    let mut player = player_query.get_single_mut().unwrap();
    let aabb_ray = player.ray.ray;
    let mut dist = player.ray.max;
    for wall in &query {
        gizmos.line_2d(wall.start, wall.end, Color::WHITE);
        if (intersects(
            aabb_ray.origin,
            aabb_ray.get_point(dist),
            wall.start,
            wall.end,
        )) {
            let intersection = get_intersection(
                aabb_ray.origin,
                aabb_ray.get_point(dist),
                wall.start,
                wall.end,
            );
            if let Some(intersection) = intersection {
                gizmos.circle_2d(intersection, 10., Color::GREEN);
                intersections.push(intersection);
            }
        }
        gizmos.circle_2d(aabb_ray.get_point(dist), 10., Color::BLUE);
    }

    if !intersections.is_empty() {
        let closest = intersections
            .iter()
            .min_by(|a, b| {
                a.distance(aabb_ray.origin)
                    .partial_cmp(&b.distance(aabb_ray.origin))
                    .unwrap()
            })
            .unwrap();
        dist = aabb_ray.origin.distance(*closest);
    }

    let player = Player {
        ray: RayCast2d::from_ray(aabb_ray, dist),
    };

    draw_ray(&mut gizmos, &player.ray);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player {
            ray: RayCast2d::from_ray(
                Ray2d {
                    origin: Vec2::from_array([0.0, 0.0]).normalize(),
                    direction: Direction2d::new_unchecked(Vec2::new(0.1, 0.1).normalize()),
                },
                150.0,
            ),
        },
    ));
    commands.spawn(TextBundle::from_section(
        "
    Tekstiä",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        },
    ));
    let mut map = [[0u8; 10]; 10];
    map[0][0] = 1;
    map[0][1] = 1;
    map[0][2] = 1;
    map[0][3] = 1;
    map[0][4] = 1;
    map[0][5] = 1;
    map[0][6] = 1;
    map[0][7] = 1;
    map[0][8] = 1;
    map[0][9] = 1;
    map[1][0] = 1;
    map[2][0] = 1;
    map[3][0] = 1;
    map[4][0] = 1;
    map[5][0] = 1;
    map[6][0] = 1;
    map[7][0] = 1;
    map[8][0] = 1;
    map[9][0] = 1;
    map[9][1] = 1;
    map[9][2] = 1;
    map[9][3] = 1;
    map[9][4] = 1;
    map[9][5] = 1;
    map[9][6] = 1;
    map[9][7] = 1;
    map[9][8] = 1;
    map[9][9] = 1;
    map[1][9] = 1;
    map[2][9] = 1;
    map[3][9] = 1;
    map[4][9] = 1;
    map[5][9] = 1;
    map[6][9] = 1;
    map[7][9] = 1;
    map[8][9] = 1;

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

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn draw_gizmo(mut gizmos: Gizmos, mut my_gizmoz: Gizmos<MyRoundGizmos>, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 10.0;

    //Wall
    gizmos.line_2d(
        Vec2::from_array([0.5, -1.]),
        Vec2::from_array([0.5, 1.0]),
        Color::RED,
    );
    gizmos.line_2d(
        Vec2::from_array([-100., -100.]),
        Vec2::from_array([0.5, 1.0]),
        Color::RED,
    );
}

pub struct HelloPlugin;

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    //mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            let direction = Vec2::new(time.elapsed_seconds().cos(), time.elapsed_seconds().sin());
            //transform.ray.ray.direction = Direction2d::new_unchecked(direction);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            //direction.x += 1.;
            //let res = transform
            //    .ray
            //    .ray
            //    .direction
            //    .rotate(Vec2::from_array([1.01, 1.01]));
            //println!("res: {:?}", res.normalize());
            //println!("direction: {:?}", transform.ray.ray.direction);
            let x = transform.ray.ray.direction.x;
            let y = transform.ray.ray.direction.y;

            //let new_direction =
            //    Vec2::new(transform.ray.ray.direction.x, transform.ray.ray.direction.y)
            //        * Vec2::new(1.1, 1.1);
            //println!("new_direction: {:?}", new_direction);
            //transform.ray.ray.direction = Direction2d::new_unchecked(new_direction.normalize());
        }
        //transform.ray.ray.direction += direction.extend(0.0) * time.delta_seconds();
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, (setup, add_walls))
            .add_systems(FixedUpdate, move_player)
            .add_systems(Update, ((draw_walls).chain(),));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .add_plugins(HelloPlugin)
        .run();
}
