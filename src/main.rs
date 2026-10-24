use std::path::{Path, PathBuf};

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::input_map::InputMap;

use crate::{
    camera::CameraPlugin,
    editor::ShipEditorPlugin,
    hud::{HudPlugin, player_hud},
    input::{GameAction, PlayerInputPlugin},
    main_menu::MainMenuPlugin,
    ships::{DumbFireShipController, ShipHull, ShipsPlugin, Spaceship},
};

mod camera;
mod editor;
mod hud;
mod input;
mod main_menu;
mod ships;

fn config_file(file: impl AsRef<Path>) -> PathBuf {
    const GAME_NAME: &str = "untitled-space-game";

    #[cfg(not(target_family = "wasm"))]
    let config_dir = dirs::config_dir().unwrap().join(GAME_NAME);

    #[cfg(target_family = "wasm")]
    let config_dir = Path::new("local").join("configuration");

    config_dir.join(file)
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    ShipEditor,
}

fn main() {
    // Bevy init
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin));

    // Physics Setup
    app.add_plugins(
        PhysicsPlugins::default()
            .with_length_unit(100.0)
            .set(PhysicsInterpolationPlugin::interpolate_all()),
    )
    .insert_resource(Gravity(Vec2::ZERO));

    // Project plugins and systems
    app.init_state::<GameState>()
        .add_plugins((
            MainMenuPlugin,
            CameraPlugin,
            ShipEditorPlugin,
            ShipsPlugin,
            PlayerInputPlugin,
            HudPlugin,
        ))
        .add_systems(OnEnter(GameState::Game), scene.spawn());

    app.run();
}

fn scene() -> impl SceneList {
    let mut asteriods = Vec::new();

    for _ in 0..100 {
        let radius = rand::random_range(10.0..30.0);
        let hp = radius * 3.0;

        let position = Vec2::new(
            rand::random_range(-3000.0..3000.0),
            rand::random_range(-3000.0..3000.0),
        );

        let scene = bsn! {
            DespawnOnExit<GameState>(GameState::Game)
            @asteroid(radius, hp)
            Position(position)
        };

        asteriods.push(scene);
    }

    bsn_list! {
        DespawnOnExit<GameState>(GameState::Game)
        #PlayerShip
        @player()
        --
        DespawnOnExit<GameState>(GameState::Game)
        #EnemyShip
        @enemy()
        --
        {asteriods}
        --
        DespawnOnExit<GameState>(GameState::Game)
        @camera::MainCamera {
            follow: Option::Some(#PlayerShip)
        }
        --
        DespawnOnExit<GameState>(GameState::Game)
        @player_hud(#PlayerShip)
    }
}

fn asteroid(radius: f32, hp: f32) -> impl Scene {
    let shape = Circle::new(radius);

    bsn! {
        ShipHull {
            max_hp: hp,
        }

        Transform::from_xyz(0.0, 0.0, 0.0)
        Mesh2d(asset_value(shape))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.75, 0.75, 0.75)))

        RigidBody::Dynamic
        Collider::from(shape)
        ColliderDensity(1.0)
    }
}

fn player() -> impl Scene {
    let ship_shape = Triangle2d::new(
        Vec2 { x: -50.0, y: -50.0 },
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 50.0, y: -50.0 },
    );

    bsn! {
        Spaceship
        InputMap<GameAction>
        ShipHull {
            max_hp: 1000.0,
        }

        Transform::from_xyz(0.0, 0.0, 0.0)
        Mesh2d(asset_value(ship_shape))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(0.0, 0.5, 1.0)))

        RigidBody::Dynamic
        Collider::from(ship_shape)

        Children [
            Name("Reactor MK5")
            ships::PowerSource::new(600.0)
            --
            Name("Battery 100H4")
            ships::PowerStorage::new(1000.0)
            --
            Name("Engine XN-2000")
            ships::ForwardEngine {
                forward_thrust: 500000.0,
                power_draw: 700.0
            }
            --
            Name("Engine RH-500")
            ships::ReverseEngine {
                reverse_thrust: 100000.0,
                power_draw: 200.0
            }
            --
            Name("Rcs RF-150")
            ships::TurningEngine {
                turn_torque: 1000000000.0,
                power_draw: 50.0
            }
            --
            Name("Plasma Cannon")
            ships::ProjectileWeapon {
                velocity: 500.0,
                frequency: 4.0,
                power_draw: 100.0,
            }
        ]
    }
}

fn enemy() -> impl Scene {
    let ship_shape = Triangle2d::new(
        Vec2 { x: -50.0, y: -50.0 },
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 50.0, y: -50.0 },
    );

    bsn! {
        Spaceship
        DumbFireShipController
        ShipHull {
            max_hp: 1000.0,
        }

        Transform::from_xyz(150.0, 300.0, 0.0)
        Mesh2d(asset_value(ship_shape))
        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 0.0, 0.0)))

        RigidBody::Dynamic
        Collider::from(ship_shape)

        Children [
            Name("Reactor MK5")
            ships::PowerSource::new(600.0)
            --
            Name("Battery 100H4")
            ships::PowerStorage::new(1000.0)
            --
            Name("Plasma Cannon")
            ships::ProjectileWeapon {
                velocity: 500.0,
                frequency: 4.0,
                power_draw: 100.0,
            }
        ]
    }
}
