use bevy::prelude::*;
use bevy_persistent::prelude::*;
use leafwing_input_manager::{
    Actionlike,
    input_map::InputMap,
    input_processing::WithDualAxisProcessingPipelineExt,
    plugin::InputManagerPlugin,
    user_input::{MouseScrollAxis, VirtualAxis, VirtualDPad},
};
use serde::{Deserialize, Serialize};

use crate::config_file;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<PlayerShipAction>::default(),
            InputManagerPlugin::<EditorAction>::default(),
            InputManagerPlugin::<ViewAction>::default(),
        ));

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
                init_game_keybinds,
                init_editor_keybinds,
                init_view_keybinds,
                update_keybinds.run_if(resource_changed::<Persistent<Keybinds>>),
            ),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum PlayerShipAction {
    #[actionlike(Axis)]
    Thrust,
    #[actionlike(Axis)]
    Turn,

    FirePrimary,
    FireMissiles,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum EditorAction {
    Place,
    RotateNext,
    RotatePrev,
    Remove,

    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
    Slot9,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum ViewAction {
    /// Move the camera at a set speed in the axis direction
    #[actionlike(DualAxis)]
    MoveCamera,
    /// When pressed dragging the mouse cursor will pan the camera
    PanCamera,
    /// Zoom the camera by the delta in the axis value
    #[actionlike(Axis)]
    ZoomCamera,

    OpenMap,
    OpenPauseMenu,
}

impl PlayerShipAction {}

#[derive(Resource, Serialize, Deserialize)]
pub struct Keybinds {
    game: InputMap<PlayerShipAction>,
    editor: InputMap<EditorAction>,
    view: InputMap<ViewAction>,
}

impl Keybinds {
    fn default_ship_keybinds() -> InputMap<PlayerShipAction> {
        let mut input_map = InputMap::default();

        //TODO: leafwing-input-manager fails to deserialize GamepadAxis values
        //input_map.insert_axis(PlayerAction::Thrust, GamepadAxis::LeftStickY);
        //input_map.insert_axis(PlayerAction::Turn, GamepadAxis::LeftStickX);
        input_map.insert(PlayerShipAction::FirePrimary, GamepadButton::West);
        input_map.insert(PlayerShipAction::FireMissiles, GamepadButton::South);

        input_map.insert_axis(PlayerShipAction::Thrust, VirtualAxis::ws());
        input_map.insert_axis(PlayerShipAction::Turn, VirtualAxis::ad());
        input_map.insert(PlayerShipAction::FirePrimary, MouseButton::Left);
        input_map.insert(PlayerShipAction::FireMissiles, MouseButton::Right);

        input_map
    }

    fn default_editor_keybinds() -> InputMap<EditorAction> {
        let mut input_map = InputMap::default();

        input_map.insert(EditorAction::Place, MouseButton::Left);
        input_map.insert(EditorAction::Remove, MouseButton::Right);

        input_map.insert(EditorAction::RotatePrev, KeyCode::KeyQ);
        input_map.insert(EditorAction::RotateNext, KeyCode::KeyE);

        input_map.insert(EditorAction::Slot0, KeyCode::Digit0);
        input_map.insert(EditorAction::Slot1, KeyCode::Digit1);
        input_map.insert(EditorAction::Slot2, KeyCode::Digit2);
        input_map.insert(EditorAction::Slot3, KeyCode::Digit3);
        input_map.insert(EditorAction::Slot4, KeyCode::Digit4);
        input_map.insert(EditorAction::Slot5, KeyCode::Digit5);
        input_map.insert(EditorAction::Slot6, KeyCode::Digit6);
        input_map.insert(EditorAction::Slot7, KeyCode::Digit7);
        input_map.insert(EditorAction::Slot8, KeyCode::Digit8);
        input_map.insert(EditorAction::Slot9, KeyCode::Digit9);

        input_map
    }

    fn default_view_keybinds() -> InputMap<ViewAction> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(
            ViewAction::MoveCamera,
            VirtualDPad::wasd().sensitivity(300.0),
        );
        input_map.insert_axis(ViewAction::ZoomCamera, MouseScrollAxis::Y);
        input_map.insert(ViewAction::PanCamera, MouseButton::Middle);

        input_map
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            game: Self::default_ship_keybinds(),
            editor: Self::default_editor_keybinds(),
            view: Self::default_view_keybinds(),
        }
    }
}

fn update_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut game_input_maps: Query<&mut InputMap<PlayerShipAction>>,
    mut editor_input_maps: Query<&mut InputMap<EditorAction>>,
    mut view_input_maps: Query<&mut InputMap<ViewAction>>,
) {
    game_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.game.clone();
    });

    editor_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.editor.clone();
    });

    view_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.view.clone();
    });
}

fn init_game_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut player_input_maps: Query<
        &mut InputMap<PlayerShipAction>,
        Added<InputMap<PlayerShipAction>>,
    >,
) {
    player_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.game.clone();
    });
}

fn init_editor_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut editor_input_maps: Query<&mut InputMap<EditorAction>, Added<InputMap<EditorAction>>>,
) {
    editor_input_maps.iter_mut().for_each(|mut map| {
        *map = keybinds.editor.clone();
    });
}

fn init_view_keybinds(
    keybinds: Res<Persistent<Keybinds>>,
    mut editor_input_maps: Query<(&Name, &mut InputMap<ViewAction>), Added<InputMap<ViewAction>>>,
) {
    editor_input_maps.iter_mut().for_each(|mut map| {
        *map.1 = keybinds.view.clone();
    });
}
