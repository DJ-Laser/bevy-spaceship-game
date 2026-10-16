use bevy::prelude::*;
use leafwing_input_manager::{
    Actionlike, input_map::InputMap, plugin::InputManagerPlugin, user_input::VirtualAxis,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Thrust,
    #[actionlike(Axis)]
    Turn,

    FirePrimary,
    FireMissiles,
}

pub fn default_player_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    input_map.insert_axis(PlayerAction::Thrust, GamepadAxis::LeftStickY);
    input_map.insert_axis(PlayerAction::Turn, GamepadAxis::LeftStickX);
    input_map.insert(PlayerAction::FirePrimary, GamepadButton::West);
    input_map.insert(PlayerAction::FireMissiles, GamepadButton::South);

    input_map.insert_axis(PlayerAction::Thrust, VirtualAxis::ws());
    input_map.insert_axis(PlayerAction::Turn, VirtualAxis::ad());
    input_map.insert(PlayerAction::FirePrimary, MouseButton::Left);
    input_map.insert(PlayerAction::FireMissiles, MouseButton::Right);

    input_map
}
