use bevy::{
    ecs::system::entity_command::clone_with_opt_out,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, Button},
};

use crate::{
    GameState,
    editor::{EditorManager, EditorPreview, ShopCatalog},
    ship::parts::Cost,
};

#[derive(SceneComponent, FromTemplate, Clone)]
#[scene(PartSelectionButtonProps)]
#[require(Button, Hovered)]
struct PartSelectionButton {
    editor: Entity,
    blueprint: Entity,
}

#[derive(Default, Clone)]
struct PartSelectionButtonProps {
    text: String,
}

impl PartSelectionButton {
    fn scene(props: PartSelectionButtonProps) -> impl Scene {
        bsn! {
          Node {
              width: Val::Px(180.0),
              height: Val::Px(32.0),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
          }
          BackgroundColor(Color::srgb(0.22, 0.22, 0.26))

          Children [
            Text::new(props.text)
            TextFont {
                font_size: FontSize::Px(12.0),
            }
          ]

          on(button_on_activate)
        }
    }
}

fn button_on_activate(
    event: On<Activate>,
    mut commands: Commands,
    buttons: Query<&PartSelectionButton>,
    mut editors: Query<&mut EditorManager>,
) {
    let Ok(button) = buttons.get(event.entity) else {
        return;
    };

    let Ok(mut editor) = editors.get_mut(button.editor) else {
        return;
    };

    if let Some(preview_id) = editor.preview_entity {
        commands.entity(preview_id).despawn();
    }

    let new_preview_id = commands.spawn((ChildOf(button.editor), EditorPreview)).id();

    commands
        .entity(button.blueprint)
        .queue(clone_with_opt_out(new_preview_id, |b| {
            b.linked_cloning(true);
            b.insert_mode(bevy::ecs::bundle::InsertMode::Keep);
        }));

    editor.preview_entity = Some(new_preview_id);
}

pub fn populate_shop_ui(
    mut commands: Commands,
    shop_children: Single<&Children, With<ShopCatalog>>,
    editor: Single<Entity, With<EditorManager>>,
    parts: Query<(Entity, &Name, &Cost)>,
) {
    let editor = editor.into_inner();

    let buttons: Vec<_> = parts
        .iter_many(shop_children.into_inner())
        .matched()
        .map(|(id, name, cost)| {
            bsn! {
              @PartSelectionButton {
                @text: {format!("{} (${})", name, cost.0)},
                blueprint: id,
                editor,
              }
            }
        })
        .collect();

    commands.spawn_scene(bsn! {
      DespawnOnExit<GameState>(GameState::ShipEditor)
      Node {
          position_type: PositionType::Absolute,
          left: Val::Px(12.0),
          top: Val::Px(12.0),
          flex_direction: FlexDirection::Column,
          row_gap: Val::Px(6.0),
          padding: UiRect::all(Val::Px(10.0)),
      }
      BackgroundColor(Color::srgb(0.14, 0.14, 0.18))
      Children [
        Text::new("Available Parts")
        TextFont {
            font_size: FontSize::Px(15.0),
        }
        --
        {buttons}
      ]
    });
}

pub fn update_buttons(
    mut buttons: Query<
        (&Hovered, Has<Pressed>, &mut BackgroundColor),
        (Changed<Hovered>, Changed<Pressed>, With<Button>),
    >,
) {
    buttons.iter_mut().for_each(
        |(hovered, pressed, mut bg_color)| match (hovered.0, pressed) {
            (_, true) => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.6, 1.0));
            }
            (true, false) => {
                println!("Hovered");
                *bg_color = BackgroundColor(Color::srgb(0.28, 0.28, 0.34));
            }
            (false, false) => {
                *bg_color = BackgroundColor(Color::srgb(0.22, 0.22, 0.26));
            }
        },
    );
}
