use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::PlayerAction,
    ships::{
        ForwardEngine, InactiveSystem, ReverseEngine, Spaceship, TurningEngine,
        engines::EngineAction,
        weapons::{ProjectileWeapon, WeaponAction},
    },
};

pub fn control_player_ship(
    time: Res<Time>,
    ships: Query<
        (
            &Children,
            &ActionState<PlayerAction>,
            Forces,
            &ComputedAngularInertia,
        ),
        With<Spaceship>,
    >,
    engines: Query<
        AnyOf<(&ForwardEngine, &ReverseEngine, &TurningEngine)>,
        Without<InactiveSystem>,
    >,
    mut engine_actions: Query<&mut ActionState<EngineAction>>,
    mut primary_weapon_actions: Query<&mut ActionState<WeaponAction>, With<ProjectileWeapon>>,
) {
    ships
        .iter()
        .for_each(|(children, player_actions, forces, angular_inertia)| {
            let mut total_forward_thrust = 0.0;
            let mut total_reverse_thrust = 0.0;
            let mut total_turn_torque = 0.0;

            engines.iter_many(children).matched().for_each(
                |(forward_engine, reverse_engine, turn_engine)| {
                    if let Some(forward_engine) = forward_engine {
                        total_forward_thrust += forward_engine.forward_thrust;
                    }

                    if let Some(reverse_engine) = reverse_engine {
                        total_reverse_thrust += reverse_engine.reverse_thrust;
                    }

                    if let Some(turn_engine) = turn_engine {
                        total_turn_torque += turn_engine.turn_torque;
                    }
                },
            );

            let mut iter = engine_actions.iter_many_mut(children).matched();
            while let Some(mut engine_actions) = iter.fetch_next() {
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
            }

            let mut iter = primary_weapon_actions.iter_many_mut(children).matched();
            while let Some(mut weapon_actions) = iter.fetch_next() {
                weapon_actions.set_button_value(
                    &WeaponAction::Fire,
                    player_actions.button_value(&PlayerAction::FirePrimary),
                );
            }
        });
}
