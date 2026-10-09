use bevy::prelude::*;

use crate::physics::{PhysicsPlugin, Position, Velocity};

mod physics;

#[derive(Component)]
struct Spaceship;

#[derive(Component)]
struct Star;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Spaceship,
        Camera2d,
        Position(Vec2::default()),
        Velocity(Vec2::new(0.0, 10.0)),
        Transform::from_xyz(0.0, 0.0, 100.0),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2 { x: -50.0, y: -50.0 },
            Vec2 { x: 0.0, y: 50.0 },
            Vec2 { x: 50.0, y: -50.0 },
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
    ));

    for _ in 0..1000 {
        let position = Vec2::new(
            rand::random_range(-1000.0..1000.0),
            rand::random_range(-1000.0..1000.0),
        );

        commands.spawn((
            Star,
            Position(position),
            Transform::default(),
            Mesh2d(meshes.add(Circle::new(rand::random_range(3.0..=7.0) / 2.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        ));
    }
}
