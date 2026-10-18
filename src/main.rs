use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    camera::CameraPlugin,
    input::PlayerInputPlugin,
    ships::{InactiveSystem, PowerStorage, ShipHull, ShipsPlugin, Spaceship},
};

mod camera;
mod input;
mod ships;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            PhysicsPlugins::default()
                .with_length_unit(100.0)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
        )
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins(CameraPlugin)
        .add_plugins(ShipsPlugin)
        .add_plugins(PlayerInputPlugin)
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, (energy_hud_text, velocity_hud_text))
        .run();
}

#[derive(Component, FromTemplate, Debug, Clone)]
#[require(Text)]
struct EnergyHud(Entity);

#[derive(Component, FromTemplate, Debug, Clone)]
#[require(Text)]
struct VelocityHud(Entity);

fn energy_hud_text(
    hud: Single<(&EnergyHud, &mut Text)>,
    spaceships: Query<&Children, With<Spaceship>>,
    energy_storages: Query<&PowerStorage, Without<InactiveSystem>>,
) {
    let (ship_id, mut text) = hud.into_inner();
    let mut total_capacity = 0.0;
    let mut total_stored = 0.0;

    let Ok(children) = spaceships.get(ship_id.0) else {
        return;
    };

    energy_storages
        .iter_many(children)
        .matched()
        .for_each(|storage| {
            total_capacity += storage.storage_capacity;
            total_stored += storage.stored_energy;
        });

    let percentage = (total_stored / total_capacity) * 100.0;
    text.0 = format!("Energy: {percentage:.0}%");
}

fn velocity_hud_text(
    hud: Single<(&VelocityHud, &mut Text)>,
    ships: Query<(Entity, &LinearVelocity, &AngularVelocity), Without<InactiveSystem>>,
) {
    let (ship_id, mut text) = hud.into_inner();

    let Some((_, lin_vel, ang_vel)) = ships.iter().find(|(ship, _, _)| *ship == ship_id.0) else {
        return;
    };

    let linear = lin_vel.length();
    let angular = ang_vel.abs();
    text.0 = format!("Velocity: {linear:.2}\nAngular: {angular:.2}");
}

fn scene() -> impl SceneList {
    let mut asteriods = Vec::new();

    for _ in 0..100 {
        let radius = rand::random_range(10.0..30.0);
        let hp = radius * 3.0;

        let position = Vec2::new(
            rand::random_range(-3000.0..3000.0),
            rand::random_range(-3000.0..3000.0),
        );

        let scene = bsn! {
            @asteroid(radius, hp)
            Position(position)
        };

        asteriods.push(scene);
    }

    bsn_list! {
        #PlayerShip
        @player()
        --
        {asteriods}
        --
        @camera::MainCamera {
            follow: #PlayerShip
        }
        --
        #Hud
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            bottom: px(5),
            left: px(15),
        }
        Children [
            EnergyHud(#PlayerShip)
            --
            VelocityHud(#PlayerShip)
        ]
    }
}

fn asteroid(radius: f32, hp: f32) -> impl Scene {
    let shape = Circle::new(radius);

    bsn! {
        input::default_player_input_map()
        ShipHull {
            max_hp: hp,
        }

        Transform::from_xyz(0.0, 0.0, 0.0)
        Mesh2d(asset_value(shape))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.75, 0.75, 0.75)))

        RigidBody::Dynamic
        Collider::from(shape)
        ColliderDensity(1.0)
    }
}

fn player() -> impl Scene {
    let ship_shape = Triangle2d::new(
        Vec2 { x: -50.0, y: -50.0 },
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 50.0, y: -50.0 },
    );

    bsn! {
        Spaceship
        input::default_player_input_map()
        ShipHull {
            max_hp: 1000.0,
        }

        Transform::from_xyz(0.0, 0.0, 0.0)
        Mesh2d(asset_value(ship_shape))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 0.5, 1.0)))

        RigidBody::Dynamic
        Collider::from(ship_shape)

        Children [
            Name("Reactor MK5")
            ships::PowerSource::new(600.0)
            --
            Name("Battery 100H4")
            ships::PowerStorage::new(1000.0)
            --
            Name("Engine XN-2000")
            ships::ForwardEngine {
                forward_thrust: 500000.0,
                power_draw: 700.0
            }
            --
            Name("Engine RH-500")
            ships::ReverseEngine {
                reverse_thrust: 100000.0,
                power_draw: 200.0
            }
            --
            Name("Rcs RF-150")
            ships::TurningEngine {
                turn_torque: 1000000000.0,
                power_draw: 50.0
            }
            --
            Name("Plasma Cannon")
            ships::ProjectileWeapon {
                velocity: 500.0,
                frequency: 4.0,
                power_draw: 100.0,
            }
        ]
    }
}
