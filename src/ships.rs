use bevy::prelude::*;

mod movement;
mod power_system;

pub use movement::{ForwardEngine, InertialDampener, ReverseEngine, TurningEngine};
pub use power_system::{PowerConsumer, PowerSource, PowerStorage};

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                power_system::clear_power_demands,
                (
                    movement::request_power::<ForwardEngine>,
                    movement::request_power::<ReverseEngine>,
                    movement::request_power::<TurningEngine>,
                    movement::request_power::<InertialDampener>,
                ),
                power_system::allocate_ship_power,
                (
                    movement::apply_movement::<ForwardEngine>,
                    movement::apply_movement::<ReverseEngine>,
                    movement::apply_movement::<TurningEngine>,
                    movement::apply_movement::<InertialDampener>,
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
