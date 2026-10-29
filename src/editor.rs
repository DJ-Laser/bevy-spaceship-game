pub mod grid;
mod ui;

use bevy::{
    prelude::*,
    ui_widgets::{Activate, ActivateOnPress, Button},
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{
    GameState,
    editor::grid::{GridManager, snap_cursor},
    input::{EditorAction, GameAction},
    player_ship,
    ship::{
        controller::PlayerShipController,
        part_components::InactivePart,
        parts::{self, Cost},
    },
};

pub struct ShipEditorPlugin;

impl Plugin for ShipEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::ShipEditor),
            (editor_scene.spawn(), ui::populate_shop_ui).chain(),
        );

        app.add_systems(OnExit(GameState::ShipEditor), remove_editor_components);

        app.add_systems(
            Update,
            (
                (update_cursor, update_preview_transform).chain(),
                place_part,
                ui::update_buttons,
                grid::draw_grid_boxes,
            )
                .run_if(in_state(GameState::ShipEditor)),
        );
    }
}

#[derive(Component, Clone, FromTemplate)]
#[require(GridManager)]
struct EditorManager {
    shop_entity: Entity,
    cursor_position: Option<Vec2>,
    #[template(built_in)]
    preview_entity: Option<Entity>,
}

#[derive(Component, Clone, FromTemplate)]
#[require(Visibility::Hidden, InactivePart, Transform)]
struct EditorPreview;

#[derive(Component, Clone, FromTemplate)]
#[require(Visibility::Hidden, Transform, Pickable::IGNORE, DespawnOnExit::<GameState>(GameState::ShipEditor))]
struct ShopCatalog;

fn editor_scene() -> impl SceneList {
    bsn_list! {
        @crate::camera::MainCamera {
            follow: Option::Some(#PlayerShip)
        }
        --
        DespawnOnExit<GameState>(GameState::ShipEditor)
        #Shop
        ShopCatalog
        Children [
            {parts::lists::common_parts()}
        ]
        --
        #PlayerShip
        @player_ship()
        DespawnOnEnter<GameState>(GameState::MainMenu)
        InputMap<GameAction>
        PlayerShipController

        InputMap<EditorAction>
        EditorManager {
            shop_entity: #Shop,
            preview_entity: Option::Some(#Preview),
        }
        --
        DespawnOnExit<GameState>(GameState::ShipEditor)
        Button
        ActivateOnPress
        Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            right: px(0),
            bottom: px(0),
        }
        BackgroundColor(Color::srgb(0.22, 0.22, 0.26))
        Children [
            ImageNode {
            image: "textures/icons/right.png",
            }
            Node {
                width: px(50),
                position_type: PositionType::Absolute,
                left: px(10),
            }
            --
            Text::new("Play")
            TextFont {
            font_size: FontSize::Px(33.0),
            }
        ]

        on(|_: On<Activate>, mut game_state: ResMut<NextState<GameState>>| {
            game_state.set(GameState::Game);
          })
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

            let snapped = snap_cursor(cursor);

            *visibility = Visibility::Inherited;
            transform.translation.x = snapped.x;
            transform.translation.y = snapped.y;
        }
    });
}

fn place_part(
    mut commands: Commands,
    mut editors: Query<(&mut EditorManager, &ActionState<EditorAction>)>,
    previews: Query<(&Name, &Cost), With<EditorPreview>>,
) {
    editors.iter_mut().for_each(|(mut editor, actions)| {
        if let Some(preview_id) = editor.preview_entity
            && actions.just_pressed(&EditorAction::Place)
        {
            let Ok((name, cost)) = previews.get(preview_id) else {
                editor.preview_entity = None;
                commands.entity(preview_id).despawn();
                return;
            };

            println!("Adding {} for ${}", name, cost.0);

            editor.preview_entity = None;
            commands
                .entity(preview_id)
                .remove::<(EditorPreview, Visibility, InactivePart, Pickable)>();
        };
    });
}

fn remove_editor_components(mut commands: Commands, editors: Query<Entity, With<EditorManager>>) {
    editors.iter().for_each(|editor_id| {
        commands.entity(editor_id).remove::<EditorManager>();
    });
}
