use std::fmt::Display;
use std::time::Duration;

use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
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

const RECTANGLE_DIMENSIONS: (f32, f32) = (20 as f32, 20 as f32);
const BORDER_SIZE: f32 = 1 as f32;
const SNAKE_MOVE_DELAY: Duration = Duration::from_secs(1);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn to_bevy_pos(&self) -> (f32, f32) {
        (
            // x * rect + x * border = x * (rect + border)
            self.x as f32 * (RECTANGLE_DIMENSIONS.0 + BORDER_SIZE),
            self.y as f32 * (RECTANGLE_DIMENSIONS.1 + BORDER_SIZE),
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("X: {} Y: {}", self.x, self.y))
    }
}

#[derive(Component)]
struct Snake {
    head: Head,
    body: Vec<BodyPart>,
    timer: Timer,
}

#[derive(Component)]
struct Head {
    position: Position,
    direction: Direction,
}

#[derive(Component)]
struct BodyPart(Position);

fn make_snake(mut commands: Commands) {
    commands.spawn(Snake {
        head: Head {
            position: Position { x: 1, y: 1 },
            direction: Direction::Right,
        },
        body: vec![
            BodyPart(Position { x: 2, y: 1 }),
            BodyPart(Position { x: 3, y: 1 }),
        ],
        timer: Timer::new(SNAKE_MOVE_DELAY, bevy::time::TimerMode::Repeating),
    });
}

fn print_positions(query: Query<&Snake>) {
    let snake = query.iter().next().unwrap();
    println!("Head: {}", snake.head.position);
    for bodypart in &snake.body {
        println!("BodyPart: {}", bodypart.0);
    }
}

fn move_snake(mut query: Query<&mut Snake>, time: Res<Time>) {
    let snake = &mut query.iter_mut().next().unwrap();

    if !snake.timer.tick(time.delta()).finished() {
        return;
    }

    match snake.head.direction {
        Direction::Up => {
            snake.head.position.y += 1;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.y += 1;
            }
        }
        Direction::Down => {
            snake.head.position.y += 1;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.y += 1;
            }
        }
        Direction::Left => {
            snake.head.position.x -= 1;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.x -= 1;
            }
        }
        Direction::Right => {
            snake.head.position.x += 1;
            for bodypart in snake.body.iter_mut() {
                bodypart.0.x += 1;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Snake>,
) {
    let snake = query.iter().next().unwrap();

    let handle = Mesh2dHandle(meshes.add(Rectangle::new(
        RECTANGLE_DIMENSIONS.0,
        RECTANGLE_DIMENSIONS.1,
    )));

    let pos = snake.head.position.to_bevy_pos();

    commands.spawn(MaterialMesh2dBundle {
        mesh: handle,
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(pos.0, pos.1, 1 as f32),
        ..default()
    });

    for bodypart in &snake.body {
        let handle = Mesh2dHandle(meshes.add(Rectangle::new(
            RECTANGLE_DIMENSIONS.0,
            RECTANGLE_DIMENSIONS.1,
        )));

        let pos = bodypart.0.to_bevy_pos();

        commands.spawn(MaterialMesh2dBundle {
            mesh: handle,
            material: materials.add(Color::DARK_GREEN),
            transform: Transform::from_xyz(pos.0, pos.1, 1 as f32),
            ..default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, make_snake)
        .add_systems(Update, render)
        .add_systems(Update, move_snake)
        .run();
}
