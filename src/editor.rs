use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{
    GameState,
    input::EditorAction,
    player_ship,
    ship::{controller::DumbFireShipController, part_components::InactivePart, parts},
};

pub struct ShipEditorPlugin;

impl Plugin for ShipEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ShipEditor), editor_scene.spawn());

        app.add_systems(
            Update,
            (
                (update_cursor, update_preview_transform).chain(),
                place_part,
            )
                .run_if(in_state(GameState::ShipEditor)),
        );
    }
}

#[derive(Component, Clone, FromTemplate)]
struct EditorManager {
    shop_entity: Entity,
    cursor_position: Option<Vec2>,
    #[template(built_in)]
    preview_entity: Option<Entity>,
}

#[derive(Component, Clone, FromTemplate)]
#[require(Visibility, InactivePart, Transform)]
struct EditorPreview;

fn editor_scene() -> impl SceneList {
    bsn_list! {
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @crate::camera::MainCamera
        --
        #Shop
        Visibility::Hidden
        Children [
            {parts::lists::common_parts()}
        ]
        --
        #Player
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @player_ship()
        DumbFireShipController

        InputMap<EditorAction>
        EditorManager {
            shop_entity: #Shop,
            preview_entity: Option::Some(#Preview),
        }

        Children [
            #Preview
            @parts::reactor_mk5()
            EditorPreview
        ]
        --
    }
}

fn update_cursor(
    mut move_events: MessageReader<PointerMove>,
    mut editors: Query<(&mut EditorManager, &GlobalTransform)>,
) {
    let Some(hit_pos) = move_events.read().last().and_then(|e| e.hit.position) else {
        return;
    };

    editors.iter_mut().for_each(|(mut editor, transform)| {
        let relative_translation = transform.affine().inverse().transform_point3(hit_pos);

        editor.cursor_position = Some(relative_translation.truncate());
    });
}

fn update_preview_transform(
    editors: Query<&EditorManager>,
    mut previews: Query<(&mut Visibility, &mut Transform), With<EditorPreview>>,
) {
    editors.iter().for_each(|editor| {
        if let Some(preview_id) = editor.preview_entity
            && let Ok((mut visibility, mut transform)) = previews.get_mut(preview_id)
        {
            let Some(cursor) = editor.cursor_position else {
                *visibility = Visibility::Hidden;
                return;
            };

            *visibility = Visibility::Inherited;
            transform.translation.x = cursor.x;
            transform.translation.y = cursor.y;
            println!("{:?}", editor.cursor_position);
        }
    });
}

fn place_part(
    mut commands: Commands,
    mut editors: Query<(Entity, &mut EditorManager, &ActionState<EditorAction>)>,
) {
    editors.iter_mut().for_each(|(editor_id, editor, actions)| {
        if let Some(preview_id) = editor.preview_entity
            && actions.just_pressed(&EditorAction::Place)
        {
            commands.entity(editor_id).add_child(preview_id);

            commands
                .entity(preview_id)
                .remove::<(EditorPreview, Visibility, InactivePart)>();
        };
    });
}
