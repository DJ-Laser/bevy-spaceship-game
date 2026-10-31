pub mod grid;
mod preview;
mod ui;

use bevy::{
    prelude::*,
    ui_widgets::{Activate, ActivateOnPress, Button},
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{
    GameState,
    editor::{
        grid::{GridFootprint, GridManager},
        preview::{EditorPreview, HasPreview},
    },
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
                (update_cursor, update_preview).chain(),
                place_part,
                ui::update_buttons,
                grid::draw_grid_boxes,
            )
                .run_if(in_state(GameState::ShipEditor)),
        );
    }
}

#[derive(Component, Clone, FromTemplate)]
#[require(GridManager, Visibility)]
struct EditorManager {
    shop_entity: Entity,
}

#[derive(Component, Clone, FromTemplate)]
#[require(Visibility::Hidden, Transform, Pickable::IGNORE, DespawnOnExit::<GameState>(GameState::ShipEditor))]
struct ShopCatalog;

fn editor_scene() -> impl SceneList {
    bsn_list! {
        @crate::camera::MainCamera {
            follow: Option::None
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
    mut editors: Query<(&mut HasPreview, &GridManager, &GlobalTransform), With<EditorManager>>,
    previews: Query<&GridFootprint, With<EditorPreview>>,
) {
    let Some(hit_pos) = move_events.read().last().and_then(|e| e.hit.position) else {
        return;
    };

    editors
        .iter_mut()
        .for_each(|(mut editor, grid, transform)| {
            let relative_translation = transform.affine().inverse().transform_point3(hit_pos);
            let Ok(preview) = previews.get(editor.preview()) else {
                editor.set_cursor_none();
                return;
            };

            editor.set_cursor(relative_translation.truncate(), grid, preview);
        });
}

fn update_preview(
    editors: Query<&HasPreview, With<EditorManager>>,
    mut previews: Query<(&mut Visibility, &mut Transform), With<EditorPreview>>,
) {
    editors.iter().for_each(|editor| {
        if let Ok((mut visibility, mut transform)) = previews.get_mut(editor.preview()) {
            let Some(pos) = editor.preview_translation() else {
                *visibility = Visibility::Hidden;
                return;
            };

            *visibility = Visibility::Inherited;
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    });
}

fn place_part(
    mut commands: Commands,
    mut editors: Query<
        (&HasPreview, &mut GridManager, &ActionState<EditorAction>),
        With<EditorManager>,
    >,
    previews: Query<(Entity, &GridFootprint, &Name, &Cost), With<EditorPreview>>,
) {
    editors.iter_mut().for_each(|(editor, mut grid, actions)| {
        let preview_id = editor.preview();
        let Some(footprint_coord) = editor.cursor_grid_position() else {
            return;
        };

        if actions.just_pressed(&EditorAction::Place) {
            let Ok((footprint_id, footprint, name, cost)) = previews.get(preview_id) else {
                commands.entity(preview_id).despawn();
                return;
            };

            if let Some(errors) = editor.placement_errors() {
                println!("Can't place {} at {}", name, footprint_coord);
                println!("{errors:?}");

                return;
            };

            println!("Adding {} for ${} at {}", name, cost.0, footprint_coord);

            grid.place_footprint(footprint_coord, (footprint_id, footprint));

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
