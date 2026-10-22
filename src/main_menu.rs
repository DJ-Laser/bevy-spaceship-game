use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{CYAN_600, CYAN_700, CYAN_950},
    },
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, ActivateOnPress, Button},
};

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();

        app.add_systems(
            OnEnter(GameState::MainMenu),
            (
                background.spawn(),
                |mut menu_state: ResMut<NextState<MenuState>>| menu_state.set(MenuState::Main),
            ),
        );

        app.add_systems(OnEnter(MenuState::Main), main_menu.spawn());
        app.add_systems(Update, (button_style).run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

fn background() -> impl SceneList {
    let camera_pos = Vec3::new(
        rand::random_range(-10000.0..10000.0),
        rand::random_range(-10000.0..10000.0),
        0.0,
    );

    bsn_list! {
      DespawnOnExit<GameState>(GameState::MainMenu)
      @crate::camera::MainCamera
      Transform::from_translation(camera_pos)
    }
}

const NORMAL_BUTTON: Srgba = CYAN_700;
const HOVERED_BUTTON: Srgba = CYAN_600;
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn menu_button(icon: &'static str, text: &'static str) -> impl Scene {
    bsn! {
      Button
      ActivateOnPress
      Hovered
      Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
      }
      BackgroundColor(NORMAL_BUTTON)
      Children [
        ImageNode {
          image: icon,
        }
        Node {
            width: px(50),
            position_type: PositionType::Absolute,
            left: px(10),
        }
        --
        Text::new(text)
        TextFont {
          font_size: FontSize::Px(33.0),
        }
        TextColor(WHITE)
      ]
    }
}

fn button_style(mut buttons: Query<(&mut BackgroundColor, &Hovered, Has<Pressed>), With<Button>>) {
    buttons
        .iter_mut()
        .for_each(|(mut background_color, hovered, pressed)| {
            *background_color = match (hovered.get(), pressed) {
                (false, false) => NORMAL_BUTTON.into(),
                (true, false) => HOVERED_BUTTON.into(),
                (false, true) => PRESSED_BUTTON.into(),
                (true, true) => HOVERED_PRESSED_BUTTON.into(),
            }
        });
}

fn main_menu() -> impl Scene {
    bsn! {
      DespawnOnExit<MenuState>(MenuState::Main)
      Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: percent(100),
        height: percent(100),
      }

      Children [
        Node {
          flex_direction: FlexDirection::Column,
          align_items: AlignItems::Center,

        }
        BackgroundColor(CYAN_950)
        Children [
          Text::new("Untitled Space Game")
          TextFont {
            font_size: FontSize::Px(67.0),
          }
          TextColor(WHITE)
          Node {
              margin: UiRect::all(px(50)),
          }
          --
          @menu_button("textures/icons/right.png", "Play")
          on(|_: On<Activate>, mut game_state: ResMut<NextState<GameState>>, mut menu_state: ResMut<NextState<MenuState>>| {
            game_state.set(GameState::Game);
            menu_state.set(MenuState::Disabled);
          })
          --
          @menu_button("textures/icons/exitRight.png", "Quit")
          on(|_: On<Activate>, mut exit: MessageWriter<AppExit>| {
            exit.write(AppExit::Success);
          })
        ]
      ]
    }
}
