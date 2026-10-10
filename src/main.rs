use bevy::prelude::*;

use crate::{
    physics::{PhysicsPlugin, Position, Velocity},
    starfield::{StarfieldMaterial, StarfieldMaterialPlugin},
};

mod physics;
mod starfield;

#[derive(Component)]
struct Spaceship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(StarfieldMaterialPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut star_materials: ResMut<Assets<StarfieldMaterial>>,
) {
    commands.spawn((
        Spaceship,
        Camera2d,
        Position(Vec2::default()),
        Velocity(Vec2::new(0.0, 100.0)),
        Transform::from_xyz(0.0, 0.0, 100.0),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2 { x: -50.0, y: -50.0 },
            Vec2 { x: 0.0, y: 50.0 },
            Vec2 { x: 50.0, y: -50.0 },
        ))),
        MeshMaterial2d(color_materials.add(Color::srgb(0.0, 0.5, 1.0))),
        children![(
            Transform::from_xyz(0.0, 0.0, -100.0),
            Mesh2d(meshes.add(Rectangle::new(1e4, 1e4))),
            MeshMaterial2d(star_materials.add(StarfieldMaterial::default())),
        )],
    ));
}
