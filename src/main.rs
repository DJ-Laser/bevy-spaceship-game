use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    input::PlayerInputPlugin,
    ships::{ShipsPlugin, Spaceship},
    starfield::{StarfieldMaterial, StarfieldMaterialPlugin},
};

mod input;
mod ships;
mod starfield;

#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(StarfieldMaterialPlugin)
        .add_plugins(ShipsPlugin)
        .add_plugins(PlayerInputPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut star_materials: ResMut<Assets<StarfieldMaterial>>,
) {
    let ship_shape = Triangle2d::new(
        Vec2 { x: -50.0, y: -50.0 },
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 50.0, y: -50.0 },
    );

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 100.0),
        LinearDamping(0.1),
        AngularDamping(0.8),
        Collider::from(ship_shape),
        input::default_player_input_map(),
        Mesh2d(meshes.add(ship_shape)),
        MeshMaterial2d(color_materials.add(Color::srgb(0.0, 0.5, 1.0))),
        Spaceship,
        RigidBody::Dynamic,
        children![
            (ships::PowerSource::new(600.0),),
            (ships::PowerStorage::new(500.0 * 5.0)),
            (ships::PlasmaEngine {
                engine_max_velocity: 100000.0,
                engine_force: 500000.0,
                power_draw: 700.0
            }),
            (ships::TurningSystems {
                turn_max_speed: 60.0,
                turn_torque: 100000000.0,
                power_draw: 500.0
            })
        ],
    ));

    commands.spawn((
        MainCamera,
        children![(
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(star_materials.add(StarfieldMaterial::default())),
        )],
    ));

    for _ in 0..1000 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(rand::random_range(50.0..100.0)))),
            MeshMaterial2d(color_materials.add(Color::srgb(0.4, 0.4, 0.45))),
            Transform::from_xyz(
                rand::random_range(-2000.0..2000.0),
                rand::random_range(-2000.0..2000.0),
                0.0,
            ),
        ));
    }
}
