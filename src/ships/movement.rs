use avian2d::prelude::*;
use bevy::{
    ecs::{component::ComponentIdFor, relationship::Relationship as _},
    prelude::*,
};
use leafwing_input_manager::action_state::ActionState;

use crate::input::PlayerAction;

use super::{InactiveSystem, PowerConsumer, Spaceship};

/// Engines that propel the ship forward
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct PlasmaEngine {
    pub engine_max_velocity: f32,
    pub engine_force: f32,
    pub power_draw: f32,
}

pub fn request_engine_power(
    power_id: ComponentIdFor<PlasmaEngine>,
    spaceships: Query<(Entity, &ActionState<PlayerAction>), With<Spaceship>>,
    mut engines: Query<(&ChildOf, &PlasmaEngine, &mut PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships.iter().for_each(|(spaceship_id, actions)| {
        let throttle = actions.clamped_button_value(&PlayerAction::Accelerate);

        engines.iter_mut().for_each(|(p, engine, mut consumer)| {
            if p.get() != spaceship_id {
                return;
            }

            consumer.request_power(power_id.get(), engine.power_draw * throttle);
        });
    });
}

/// Applies physics calculations modifying thrust vectors using partial energy factors.
pub fn apply_engine_thrust(
    power_id: ComponentIdFor<PlasmaEngine>,
    mut spaceships: Query<(Entity, Forces), With<Spaceship>>,
    engines: Query<(&ChildOf, &PlasmaEngine, &PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships
        .iter_mut()
        .for_each(|(spaceship_id, mut forces)| {
            engines.iter().for_each(|(p, engine, consumer)| {
                if p.get() != spaceship_id {
                    return;
                }

                let thrust_scalar = engine.engine_force
                    * (consumer.allocated_power(power_id.get()) / engine.power_draw);
                forces.apply_local_force(Vec2::Y * thrust_scalar);
            });
        });
}

/// Engines that propel the ship forward
#[derive(Component, Debug, Default, Clone)]
#[require(PowerConsumer)]
pub struct TurningSystems {
    pub turn_max_speed: f32,
    pub turn_torque: f32,
    pub power_draw: f32,
}

pub fn request_turn_power(
    power_id: ComponentIdFor<TurningSystems>,
    spaceships: Query<(Entity, &ActionState<PlayerAction>), With<Spaceship>>,
    mut engines: Query<(&ChildOf, &TurningSystems, &mut PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships.iter().for_each(|(spaceship_id, actions)| {
        let throttle = actions.clamped_value(&PlayerAction::Bank).abs();

        engines.iter_mut().for_each(|(p, turning, mut consumer)| {
            if p.get() != spaceship_id {
                return;
            }

            consumer.request_power(power_id.get(), turning.power_draw * throttle);
        });
    });
}

/// Applies physics calculations modifying thrust vectors using partial energy factors.
pub fn apply_turn_torque(
    power_id: ComponentIdFor<TurningSystems>,
    mut spaceships: Query<(Entity, &ActionState<PlayerAction>, Forces), With<Spaceship>>,
    engines: Query<(&ChildOf, &TurningSystems, &PowerConsumer), Without<InactiveSystem>>,
) {
    spaceships
        .iter_mut()
        .for_each(|(spaceship_id, actions, mut forces)| {
            engines.iter().for_each(|(p, turning, consumer)| {
                if p.get() != spaceship_id {
                    return;
                }

                let turning_scalar = turning.turn_torque
                    * (consumer.allocated_power(power_id.get()) / turning.power_draw);
                let turning_direction = f32::signum(-actions.clamped_value(&PlayerAction::Bank));

                forces.apply_torque(turning_direction * turning_scalar);
            });
        });
}
