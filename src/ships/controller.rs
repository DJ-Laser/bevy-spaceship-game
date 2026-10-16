use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::PlayerAction,
    ships::{
        ForwardEngine, InactiveSystem, ReverseEngine, Spaceship, TurningEngine,
        engines::EngineAction,
    },
};

pub fn control_player_ship(
    time: Res<Time>,
    ships: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            Forces,
            &ComputedAngularInertia,
        ),
        With<Spaceship>,
    >,
    engines: Query<AnyOf<(&ForwardEngine, &ReverseEngine, &TurningEngine)>>,
    mut systems: Query<(&ChildOf, &mut ActionState<EngineAction>), Without<InactiveSystem>>,
) {
    ships
        .iter()
        .for_each(|(spaceship_id, player_actions, forces, angular_inertia)| {
            let mut total_forward_thrust = 0.0;
            let mut total_reverse_thrust = 0.0;
            let mut total_turn_torque = 0.0;

            engines
                .iter()
                .for_each(|(forward_engine, reverse_engine, turn_engine)| {
                    if let Some(forward_engine) = forward_engine {
                        total_forward_thrust += forward_engine.forward_thrust;
                    }

                    if let Some(reverse_engine) = reverse_engine {
                        total_reverse_thrust += reverse_engine.reverse_thrust;
                    }

                    if let Some(turn_engine) = turn_engine {
                        total_turn_torque += turn_engine.turn_torque;
                    }
                });

            systems.iter_mut().for_each(|(p, mut engine_actions)| {
                if p.get() != spaceship_id {
                    return;
                }

                engine_actions.set_value(
                    &EngineAction::Thrust,
                    player_actions.value(&PlayerAction::Thrust),
                );

                let goal_angular_velocity = 0.1
                    * (total_turn_torque * angular_inertia.inverse())
                    * player_actions.clamped_value(&PlayerAction::Turn);

                let torque_needed = angular_inertia.value()
                    * (-goal_angular_velocity - forces.angular_velocity())
                    / time.delta_secs();

                engine_actions.set_value(&EngineAction::Turn, torque_needed / total_turn_torque);
            });
        });
}
