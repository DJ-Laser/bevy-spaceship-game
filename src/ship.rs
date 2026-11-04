use bevy::prelude::*;

use crate::GameState;

pub mod controller;
pub mod part_components;
pub mod parts;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        use part_components::*;

        app.add_systems(
            FixedUpdate,
            (
                (
                    power::clear_power_demands,
                    durability::init_part_durability,
                    (
                        controller::player::control_ship,
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
                power::allocate_ship_power,
                (
                    engines::fire_engines::<ForwardEngine>,
                    engines::fire_engines::<ReverseEngine>,
                    engines::fire_engines::<TurningEngine>,
                    weapons::fire_weapons,
                ),
                durability::remove_broken_parts,
            )
                .chain(),
        );
    }
}

/// A marker component that enables the spaceship update cycle for this entity.
#[derive(Component, Debug, Default, Clone)]
pub struct Spaceship;
