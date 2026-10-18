use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::PlayerAction,
    ships::{
        ForwardEngine, InactiveSystem, ReverseEngine, Spaceship, TurningEngine,
        controller::EngineTotals,
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
            let engine_totals = EngineTotals::from_query(children, engines);

            let goal_angular_velocity = 0.1
                * (engine_totals.turn_torque * angular_inertia.inverse())
                * player_actions.clamped_value(&PlayerAction::Turn);

            let torque_needed = angular_inertia.value()
                * (-goal_angular_velocity - forces.angular_velocity())
                / time.delta_secs();

            let mut iter = engine_actions.iter_many_mut(children).matched();
            while let Some(mut engine_actions) = iter.fetch_next() {
                engine_actions.set_value(
                    &EngineAction::Thrust,
                    player_actions.value(&PlayerAction::Thrust),
                );

                engine_actions.set_value(
                    &EngineAction::Turn,
                    torque_needed / engine_totals.turn_torque,
                );
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
