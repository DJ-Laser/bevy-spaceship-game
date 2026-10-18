use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::ships::{ProjectileWeapon, Spaceship, weapons::WeaponAction};

#[derive(Component, Default, Clone)]
pub struct DumbFireShipController;

pub fn control_ship(
    ships: Query<&Children, (With<Spaceship>, With<DumbFireShipController>)>,
    mut primary_weapon_actions: Query<&mut ActionState<WeaponAction>, With<ProjectileWeapon>>,
) {
    ships.iter().for_each(|children| {
        let mut iter = primary_weapon_actions.iter_many_mut(children).matched();
        while let Some(mut weapon_actions) = iter.fetch_next() {
            weapon_actions.set_button_value(&WeaponAction::Fire, 1.0);
        }
    });
}
