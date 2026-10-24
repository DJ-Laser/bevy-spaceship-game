use bevy::prelude::*;

mod controller;
mod engines;
mod power_system;
mod weapons;

pub use controller::dumb_fire::DumbFireShipController;
pub use engines::{ForwardEngine, ReverseEngine, TurningEngine};
pub use power_system::{PowerSource, PowerStorage};
pub use weapons::{ProjectileWeapon, ShipHull};

use crate::GameState;

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (
                    power_system::clear_power_demands,
                    weapons::init_ship_hull,
                    (
                        controller::player::control_player_ship,
                        controller::dumb_fire::control_ship,
                    )
                        .run_if(in_state(GameState::Game)),
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
                weapons::remove_broken_hulls,
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
