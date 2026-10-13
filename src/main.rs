use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};

use crate::{
    camera::CameraPlugin,
    input::PlayerInputPlugin,
    ships::{InactiveSystem, PowerStorage, ShipsPlugin, Spaceship},
};

mod camera;
mod input;
mod ships;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()))
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

fn velocity_hud_text(
    hud: Single<(&VelocityHud, &mut Text)>,
    ships: Query<(Entity, &LinearVelocity), Without<InactiveSystem>>,
) {
    let (ship_id, mut text) = hud.into_inner();

    let Some((_, velocity)) = ships.iter().find(|(ship, _)| *ship == ship_id.0) else {
        return;
    };

    let magnitude = velocity.length();
    text.0 = format!("Velocity: {magnitude:.2}");
}

fn scene() -> impl SceneList {
    bsn_list! {
        #PlayerShip
        @player()
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

fn player() -> impl Scene {
    let ship_shape = Triangle2d::new(
        Vec2 { x: -50.0, y: -50.0 },
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 50.0, y: -50.0 },
    );

    bsn! {
        Spaceship
        input::default_player_input_map()

        Transform::from_xyz(0.0, 0.0, 100.0)
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
                engine_force: 500000.0,
                power_draw: 700.0
            }
            --
            Name("Engine RH-500")
            ships::ReverseEngine {
                engine_force: 100000.0,
                power_draw: 200.0
            }
            --
            Name("Rcs RF-150")
            ships::TurningEngine {
                turn_torque: 100000000.0,
                power_draw: 50.0
            }
            --
            Name("Dampers DX14")
            ships::InertialDampener {
                linear_damping_constant: 5000.0,
                angular_damping_constant: 100000000.0,
            }
        ]
    }
}
