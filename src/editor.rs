use bevy::{
    color::palettes::css::{GRAY, GREEN},
    prelude::*,
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{GameState, input::EditorAction, player};

pub struct ShipEditorPlugin;

impl Plugin for ShipEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ShipEditor), editor_scene.spawn());

        app.add_systems(
            Update,
            (update_hover, place_part).run_if(in_state(GameState::ShipEditor)),
        );
    }
}

#[derive(Component, Clone, FromTemplate)]
struct EditorPreview {
    target: Entity,
    cursor_position: Option<Vec2>,
}

fn editor_scene() -> impl SceneList {
    bsn_list! {
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @crate::camera::MainCamera
        --
        #Player
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @player()
        --
        DespawnOnExit<GameState>(GameState::ShipEditor)
        Mesh2d(asset_value(Rectangle::new(50.0, 50.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GREEN)))
        Pickable::IGNORE
        InputMap<EditorAction>
        EditorPreview {
            target: #Player
        }
    }
}

fn place_part(
    mut commands: Commands,
    previews: Query<(&ActionState<EditorAction>, &EditorPreview)>,
) {
    previews.iter().for_each(|(actions, preview)| {
        let Some(cursor_position) = preview.cursor_position else {
            return;
        };

        if !actions.just_pressed(&EditorAction::Place) {
            return;
        }

        commands.spawn_scene(bsn! {
            Transform::from_translation(cursor_position.extend(0.0))
            ChildOf({preview.target})
            Mesh2d(asset_value(Rectangle::new(50.0, 50.0)))
            MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GRAY)))
        });
    });
}

fn update_hover(
    mut move_events: MessageReader<PointerMove>,
    preview: Single<(&mut Transform, &mut EditorPreview)>,
) {
    let (mut transform, mut preview) = preview.into_inner();

    move_events.read().for_each(|event| {
        let Some(hit_pos) = event.hit.position else {
            return;
        };

        transform.translation.x = hit_pos.x;
        transform.translation.y = hit_pos.y;
        preview.cursor_position = Some(hit_pos.truncate())
    });
}
