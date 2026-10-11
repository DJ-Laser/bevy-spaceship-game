use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::{PlayerAction, PlayerInputPlugin},
    physics::{Angle, AngularVelocity, PhysicsPlugin, Position, Velocity},
    starfield::{StarfieldCamera, StarfieldMaterial, StarfieldMaterialPlugin},
};

mod input;
mod physics;
mod starfield;

#[derive(Component)]
#[require(Transform, Position, Velocity, Angle, AngularVelocity)]
struct Spaceship;

#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(StarfieldMaterialPlugin)
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
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 100.0),
        Spaceship,
        Velocity(Vec2::new(0.0, 100.0)),
        input::default_player_input_map(),
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2 { x: -50.0, y: -50.0 },
            Vec2 { x: 0.0, y: 50.0 },
            Vec2 { x: 50.0, y: -50.0 },
        ))),
        MeshMaterial2d(color_materials.add(Color::srgb(0.0, 0.5, 1.0))),
        children![(
            MainCamera,
            related!(
                StarfieldCamera[(
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(star_materials.add(StarfieldMaterial::default())),
                )]
            ),
        )],
    ));
}
