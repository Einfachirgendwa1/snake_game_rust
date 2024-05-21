use bevy::{
    app::{App, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        system::{Commands, ResMut, Resource},
    },
    math::{Quat, Vec3},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, Sprite, SpriteBundle},
    transform::{self, components::Transform},
    utils::default,
    DefaultPlugins,
};
use rand::{thread_rng, Rng};

const BLOCK_SIZE: f32 = 30.;
const BLOCK_GAPS: f32 = 1.;

#[derive(Component)]
struct SnakeBodyPart;

#[derive(Component)]
struct Apple;

#[derive(Resource)]
struct SnakeBody(Vec<SnakeBodyPart>);

impl Default for SnakeBody {
    fn default() -> Self {
        Self(vec![])
    }
}

fn transform(x: i32, y: i32) -> Transform {
    Transform {
        translation: Vec3 {
            x: x as f32 * (BLOCK_SIZE + BLOCK_GAPS),
            y: y as f32 * (BLOCK_SIZE + BLOCK_GAPS),
            z: 1.,
        },
        rotation: Quat::default(),
        scale: Vec3 {
            x: BLOCK_SIZE,
            y: BLOCK_SIZE,
            ..default()
        },
    }
}

fn random_transform() -> Transform {
    let mut rng = thread_rng();
    transform(rng.gen_range(0..10), rng.gen_range(0..10))
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Apple
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: random_transform(),
            ..default()
        },
        Apple,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SnakeBody>()
        .add_systems(Startup, setup)
        .run();
}
