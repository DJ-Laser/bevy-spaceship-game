use bevy::{
    color::palettes::{
        css::{BLUE, GRAY, GREEN, PINK, RED},
        tailwind::{CYAN_500, PINK_100, RED_500},
    },
    picking::pointer::PointerInteraction,
    prelude::*,
};

use crate::{GameState, player};

pub struct ShipEditorPlugin;

impl Plugin for ShipEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::ShipEditor), editor_scene.spawn());

        app.add_systems(
            Update,
            (draw_mesh_intersections).run_if(in_state(GameState::ShipEditor)),
        );
    }
}

#[derive(Component, Clone, FromTemplate)]
struct EditorPreview {
    target: Entity,
}

fn editor_scene() -> impl SceneList {
    bsn_list! {
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @crate::camera::MainCamera
        --
        #Player
        DespawnOnExit<GameState>(GameState::ShipEditor)
        @player()
        on(|event: On<PointerMove>, mut gizmos: Gizmos,| {
            let Some(hit_pos) = event.hit.position else {
            return;
        };

            gizmos.circle_2d(hit_pos.truncate(), 10.0, BLUE);
        })

        on(|event: On<PointerOver>, mut materials: Query<&mut MeshMaterial2d<ColorMaterial>>, mut colors: ResMut<Assets<ColorMaterial>>| {
            println!("PointerOver");
            let Ok(mut material) = materials.get_mut(event.entity) else {
                return;
            };

            material.0 = colors.add(Into::<Color>::into(BLUE));
        })

        on(|event: On<PointerOut>, mut materials: Query<&mut MeshMaterial2d<ColorMaterial>>, mut colors: ResMut<Assets<ColorMaterial>>| {
            println!("PointerOut");
            let Ok(mut material) = materials.get_mut(event.entity) else {
                return;
            };

            material.0 = colors.add(Into::<Color>::into(CYAN_500));
        })

        --
        DespawnOnExit<GameState>(GameState::ShipEditor)
        Mesh2d(asset_value(Rectangle::new(50.0, 50.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GREEN)))
        EditorPreview {
            target: #Player
        }
    }
}

fn place_part(mut commands: Commands, ship: Single<Entity>) {
    commands.spawn_scene(bsn! {
        ChildOf({*ship})
        Mesh2d(asset_value(Rectangle::new(50.0, 50.0)))
        MeshMaterial2d<ColorMaterial>(asset_value(Into::<Color>::into(GRAY)))
    });
}

fn update_hover(
    mut move_events: MessageReader<PointerMove>,
    mut preview: Single<&mut Transform, With<EditorPreview>>,
    mut gizmos: Gizmos,
) {
    move_events.read().for_each(|event| {
        let Some(hit_pos) = event.hit.position else {
            return;
        };

        preview.translation.x = hit_pos.x;
        preview.translation.y = hit_pos.y;

        //gizmos.circle_2d(hit_pos.truncate(), 10.0, RED);
    });
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(
    pointers: Query<&PointerInteraction>,
    mut gizmos: Gizmos,
    names: Query<&Name>,
) {
    for (entity, point) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(entity, hit)| Some(entity).zip(hit.position))
    {
        gizmos.circle_2d(point.truncate(), 10.0, RED_500);
        let name = names.get(*entity).ok().map(|name| name.as_str());
        println!("Hit Entity: {:?}, Position: {:?}", name, point);
    }

    gizmos.circle_2d(Vec2::default(), 10.0, PINK);
}
