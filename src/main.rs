use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};

use crate::{
    input::PlayerInputPlugin,
    ships::{InactiveSystem, PowerStorage, ShipsPlugin, Spaceship},
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
        .add_systems(Update, energy_hud_text)
        .run();
}

#[derive(Component)]
struct EnergyHud(Entity);

fn energy_hud_text(
    hud: Single<(&EnergyHud, &mut Text)>,
    batteries: Query<(&ChildOf, &PowerStorage), Without<InactiveSystem>>,
) {
    let (ship_id, mut text) = hud.into_inner();
    let mut total_capacity = 0.0;
    let mut total_stored = 0.0;

    batteries.iter().for_each(|(p, storage)| {
        if p.get() != ship_id.0 {
            return;
        }

        total_capacity += storage.storage_capacity;
        total_stored += storage.stored_energy;
    });

    let percentage = (total_stored / total_capacity) * 100.0;
    text.0 = format!("Energy: {percentage:.0}%");
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

    let ship_id = commands
        .spawn((
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
                (ships::PowerStorage::new(500.0 * 2.0)),
                (ships::PlasmaEngine {
                    engine_max_velocity: 100000.0,
                    engine_force: 500000.0,
                    power_draw: 700.0
                }),
                (ships::TurningSystems {
                    turn_max_speed: 60.0,
                    turn_torque: 100000000.0,
                    power_draw: 50.0
                }),
            ],
        ))
        .id();

    commands.spawn((
        Text::new("Energy: 0"),
        EnergyHud(ship_id),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
    ));

    commands.spawn((
        MainCamera,
        children![(
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(star_materials.add(StarfieldMaterial::default())),
        )],
    ));
}
