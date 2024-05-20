use std::time::Duration;

use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    math::primitives::Rectangle,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    time::{Time, Timer},
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};

const RECTANGLE_DIMENSIONS: (f32, f32) = (20., 20.);
const BORDER_SIZE: f32 = 1.;
const SNAKE_MOVE_DELAY: Duration = Duration::from_secs(1);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Snake {
    head: Head,
    body: Vec<BodyPart>,
    timer: Timer,
}

#[derive(Component)]
struct Head {
    position: Transform,
    direction: Direction,
}

#[derive(Component)]
struct BodyPart(Transform);

fn to_transform(x: i32, y: i32) -> Transform {
    Transform::from_xyz(
        x as f32 * (RECTANGLE_DIMENSIONS.0 + BORDER_SIZE),
        y as f32 * (RECTANGLE_DIMENSIONS.1 + BORDER_SIZE),
        1.,
    )
}

fn make_snake(mut commands: Commands) {
    commands.spawn(Snake {
        head: Head {
            position: to_transform(3, 1),
            direction: Direction::Right,
        },
        body: vec![BodyPart(to_transform(2, 1)), BodyPart(to_transform(1, 1))],
        timer: Timer::new(SNAKE_MOVE_DELAY, bevy::time::TimerMode::Repeating),
    });
}

fn move_snake(mut query: Query<&mut Snake>, time: Res<Time>) {
    let snake = &mut query.iter_mut().next().unwrap();

    if !snake.timer.tick(time.delta()).finished() {
        return;
    }

    match snake.head.direction {
        Direction::Up => {
            snake.head.position.translation.y += 1.;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.translation.y += 1.;
            }
        }
        Direction::Down => {
            snake.head.position.translation.y += 1.;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.translation.y += 1.;
            }
        }
        Direction::Left => {
            snake.head.position.translation.x -= 1.;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.translation.x -= 1.;
            }
        }
        Direction::Right => {
            snake.head.position.translation.x += 1.;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.translation.x += 1.;
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    query: Query<&Snake>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let snake = query.iter().next().unwrap();

    let handle = Mesh2dHandle(meshes.add(Rectangle::new(
        RECTANGLE_DIMENSIONS.0,
        RECTANGLE_DIMENSIONS.1,
    )));

    let pos = snake.head.position.translation;

    commands.spawn(MaterialMesh2dBundle {
        mesh: handle,
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(pos.x, pos.y, 1.),
        ..default()
    });

    for bodypart in &snake.body {
        let handle = Mesh2dHandle(meshes.add(Rectangle::new(
            RECTANGLE_DIMENSIONS.0,
            RECTANGLE_DIMENSIONS.1,
        )));

        let pos = bodypart.0.translation;

        commands.spawn(MaterialMesh2dBundle {
            mesh: handle,
            material: materials.add(Color::DARK_GREEN),
            transform: Transform::from_xyz(pos.x, pos.y, 1.),
            ..default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (make_snake, setup).chain())
        .add_systems(Update, move_snake)
        .run();
}
