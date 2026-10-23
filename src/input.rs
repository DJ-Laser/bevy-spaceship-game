use bevy::prelude::*;
use bevy_persistent::prelude::*;
use leafwing_input_manager::{
    Actionlike, input_map::InputMap, plugin::InputManagerPlugin, user_input::VirtualAxis,
};
use serde::{Deserialize, Serialize};

use crate::config_file;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());

        app.insert_resource(
            Persistent::<Keybinds>::builder()
                .name("key bindings")
                .format(StorageFormat::TomlPretty)
                .path(config_file("key-bindings.toml"))
                .default(Keybinds::default())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to initialize key bindings"),
        );

        app.add_systems(
            Update,
            (
                init_keybinds,
                update_keybinds.run_if(resource_changed::<Persistent<Keybinds>>),
            ),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Thrust,
    #[actionlike(Axis)]
    Turn,

    FirePrimary,
    FireMissiles,
}

impl PlayerAction {}

#[derive(Resource, Serialize, Deserialize)]
pub struct Keybinds {
    player: InputMap<PlayerAction>,
}

impl Keybinds {
    fn default_player_keybinds() -> InputMap<PlayerAction> {
        let mut input_map = InputMap::default();

        //TODO: leafwing-input-manager fails to deserialize GamepadAxis values
        //input_map.insert_axis(PlayerAction::Thrust, GamepadAxis::LeftStickY);
        //input_map.insert_axis(PlayerAction::Turn, GamepadAxis::LeftStickX);
        input_map.insert(PlayerAction::FirePrimary, GamepadButton::West);
        input_map.insert(PlayerAction::FireMissiles, GamepadButton::South);

        input_map.insert_axis(PlayerAction::Thrust, VirtualAxis::ws());
        input_map.insert_axis(PlayerAction::Turn, VirtualAxis::ad());
        input_map.insert(PlayerAction::FirePrimary, MouseButton::Left);
        input_map.insert(PlayerAction::FireMissiles, MouseButton::Right);

        input_map
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            player: Self::default_player_keybinds(),
        }
    }
}

fn update_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut player_input_maps: Query<&mut InputMap<PlayerAction>>,
) {
    player_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.player.clone();
    });
}

fn init_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut player_input_maps: Query<&mut InputMap<PlayerAction>, Added<InputMap<PlayerAction>>>,
) {
    player_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.player.clone();
    });
}
