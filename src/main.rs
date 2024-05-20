use std::fmt::Display;

use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        schedule::IntoSystemConfigs,
        system::{Commands, Query},
    },
};

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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("X: {} Y: {}", self.x, self.y))
    }
}

#[derive(Component)]
struct Snake {
    head: Head,
    body: Vec<BodyPart>,
}

#[derive(Component)]
struct Head {
    position: Position,
    direction: Direction,
}

#[derive(Component)]
struct BodyPart(Position);

fn make_snake(mut commands: Commands) {
    commands.spawn((Snake {
        head: Head {
            position: Position { x: 5, y: 5 },
            direction: Direction::Right,
        },
        body: vec![
            BodyPart(Position { x: 6, y: 5 }),
            BodyPart(Position { x: 7, y: 5 }),
        ],
    },));
}

fn print_positions(query: Query<&Snake>) {
    let snake = query.iter().next().unwrap();
    println!("Head: {}", snake.head.position);
    for bodypart in &snake.body {
        println!("BodyPart: {}", bodypart.0);
    }
}

fn move_snake(mut query: Query<&mut Snake>) {
    println!("Bewege die Schlange...");
    let snake = &mut query.iter_mut().next().unwrap();

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

fn main() {
    App::new()
        .add_systems(Startup, make_snake)
        .add_systems(
            Update,
            (print_positions, move_snake, print_positions).chain(),
        )
        .run();
}
