use bevy::prelude::*;

mod controller;
mod engines;
mod power_system;

pub use engines::{ForwardEngine, ReverseEngine, TurningEngine};
pub use power_system::{PowerConsumer, PowerSource, PowerStorage};

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
                ),
                power_system::allocate_ship_power,
                (
                    engines::fire_engine::<ForwardEngine>,
                    engines::fire_engine::<ReverseEngine>,
                    engines::fire_engine::<TurningEngine>,
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
