use bevy::prelude::*;

mod movement;
mod power_system;

pub use movement::{PlasmaEngine, TurningSystems};
pub use power_system::{PowerConsumer, PowerSource, PowerStorage};

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                power_system::clear_power_demands,
                (movement::request_engine_power, movement::request_turn_power),
                power_system::allocate_ship_power,
                (movement::apply_engine_thrust, movement::apply_turn_torque),
            )
                .chain(),
        );
    }
}

/// A marker component that enables the spaceship update cycle for this entity.
#[derive(Component)]
pub struct Spaceship;

/// A marker component indicating that this subsystem is inactive and should not participate in the ship's update cycle.
#[derive(Component)]
pub struct InactiveSystem;
