use bevy::prelude::*;

mod controller;
mod engines;
mod power_system;
mod weapons;

pub use engines::{ForwardEngine, ReverseEngine, TurningEngine};
pub use power_system::{PowerSource, PowerStorage};
pub use weapons::ProjectileWeapon;

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (
                    power_system::clear_power_demands,
                    controller::control_player_ship,
                ),
                (
                    engines::request_engine_power::<ForwardEngine>,
                    engines::request_engine_power::<ReverseEngine>,
                    engines::request_engine_power::<TurningEngine>,
                    weapons::request_weapon_power,
                ),
                power_system::allocate_ship_power,
                (
                    engines::fire_engines::<ForwardEngine>,
                    engines::fire_engines::<ReverseEngine>,
                    engines::fire_engines::<TurningEngine>,
                    weapons::fire_weapons,
                ),
            )
                .chain(),
        );
    }
}

/// A marker component that enables the spaceship update cycle for this entity.
#[derive(Component, Debug, Default, Clone)]
pub struct Spaceship;

/// A marker component indicating that this subsystem is inactive and should not participate in the ship's update cycle.
#[derive(Component, Debug, Default, Clone)]
pub struct InactiveSystem;
