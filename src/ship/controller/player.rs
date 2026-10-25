use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::GameAction,
    ship::part_components::{ProjectileWeapon, engines::EngineAction, weapons::WeaponAction},
    ship::{
        Spaceship,
        controller::EngineTotals,
        part_components::{ForwardEngine, InactivePart, ReverseEngine, TurningEngine},
    },
};

#[derive(Component, Default, Clone)]
#[require(ActionState<GameAction>)]
pub struct PlayerShipController;

pub fn control_ship(
    time: Res<Time>,
    ships: Query<
        (
            &Children,
            &ActionState<GameAction>,
            Forces,
            &ComputedAngularInertia,
        ),
        (With<Spaceship>, With<PlayerShipController>),
    >,
    engines: Query<AnyOf<(&ForwardEngine, &ReverseEngine, &TurningEngine)>, Without<InactivePart>>,
    mut engine_actions: Query<&mut ActionState<EngineAction>>,
    mut primary_weapon_actions: Query<&mut ActionState<WeaponAction>, With<ProjectileWeapon>>,
) {
    ships
        .iter()
        .for_each(|(children, player_actions, forces, angular_inertia)| {
            let engine_totals = EngineTotals::from_query(children, engines);

            let goal_angular_velocity = 0.1
                * (engine_totals.turn_torque * angular_inertia.inverse())
                * player_actions.clamped_value(&GameAction::Turn);

            let torque_needed = angular_inertia.value()
                * (-goal_angular_velocity - forces.angular_velocity())
                / time.delta_secs();

            let mut iter = engine_actions.iter_many_mut(children).matched();
            while let Some(mut engine_actions) = iter.fetch_next() {
                engine_actions.set_value(
                    &EngineAction::Thrust,
                    player_actions.value(&GameAction::Thrust),
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
                    player_actions.button_value(&GameAction::FirePrimary),
                );
            }
        });
}
