mod gizmos;
mod preview;
mod ui;

use bevy::{
    prelude::*,
    ui_widgets::{Activate, ActivateOnPress, Button},
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{
    GameState,
    editor::preview::{EditorPreview, HasPreview},
    grid::{GridFootprint, GridManager, PositionedFootprint, RotationIncrement, RotationStyle},
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
                rotate_preview,
                gizmos::draw_grid_boxes,
            )
                .run_if(in_state(GameState::ShipEditor)),
        );

        app.init_gizmo_group::<gizmos::EditorGridGizmos>();
        app.add_systems(Startup, gizmos::setup_editor_gizmos);
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
    previews: Query<(&RotationIncrement, &GridFootprint), With<EditorPreview>>,
    nodes: Query<(), With<Node>>,
) {
    let Some(event) = move_events.read().last() else {
        return;
    };

    let hit_pos = if nodes.contains(event.entity) {
        None
    } else {
        event.hit.position
    };

    editors
        .iter_mut()
        .for_each(|(mut editor, grid, transform)| {
            let Some(hit_pos) = hit_pos else {
                editor.set_cursor_none();
                return;
            };

            let relative_translation = transform.affine().inverse().transform_point3(hit_pos);
            let Ok((rotation, preview)) = previews.get(editor.preview()) else {
                editor.set_cursor_none();
                return;
            };

            editor.set_cursor(relative_translation.truncate(), grid, *rotation, preview);
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

fn rotate_preview(
    mut editors: Query<
        (&mut HasPreview, &GridManager, &ActionState<EditorAction>),
        With<EditorManager>,
    >,
    mut previews: Query<
        (
            &RotationStyle,
            &mut RotationIncrement,
            &mut Transform,
            &GridFootprint,
        ),
        With<EditorPreview>,
    >,
) {
    editors.iter_mut().for_each(|(mut editor, grid, actions)| {
        let Ok((style, mut rotation, mut transform, footprint)) =
            previews.get_mut(editor.preview())
        else {
            return;
        };

        let prev = actions.just_pressed(&EditorAction::RotatePrev);
        let next = actions.just_pressed(&EditorAction::RotateNext);

        if prev == next {
            return;
        }

        if prev {
            *rotation = rotation.prev(style);
        }

        if next {
            *rotation = rotation.next(style);
        }

        transform.rotation = Quat::from_rotation_z(rotation.rotation_radians());
        editor.update_preview(grid, *rotation, footprint);
    });
}

fn place_part(
    mut commands: Commands,
    mut editors: Query<
        (&HasPreview, &mut GridManager, &ActionState<EditorAction>),
        With<EditorManager>,
    >,
    previews: Query<
        (Entity, &RotationIncrement, &GridFootprint, &Name, &Cost),
        With<EditorPreview>,
    >,
) {
    editors.iter_mut().for_each(|(editor, mut grid, actions)| {
        let preview_id = editor.preview();
        let Some(footprint_coord) = editor.cursor_grid_position() else {
            return;
        };

        if actions.just_pressed(&EditorAction::Place) {
            let Ok((footprint_id, rotation, footprint, name, cost)) = previews.get(preview_id)
            else {
                commands.entity(preview_id).despawn();
                return;
            };

            if let Some(errors) = editor.placement_errors() {
                println!("Can't place {} at {}", name, footprint_coord);
                println!("{errors:?}");

                return;
            };

            println!("Adding {} for ${} at {}", name, cost.0, footprint_coord);

            grid.place_footprint(
                footprint_id,
                PositionedFootprint::new(footprint, footprint_coord, *rotation),
            );

            commands
                .entity(preview_id)
                .remove::<(EditorPreview, Visibility, InactivePart, Pickable)>();
        };
    });
}

fn remove_editor_components(mut commands: Commands, editors: Query<Entity, With<EditorManager>>) {
    editors.iter().for_each(|editor_id| {
        commands
            .entity(editor_id)
            .despawn_related::<HasPreview>()
            .remove::<EditorManager>();
    });
}
