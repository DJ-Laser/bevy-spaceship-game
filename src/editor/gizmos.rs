use bevy::{
    color::palettes::css::{DARK_CYAN, DARK_RED, GRAY, GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

use crate::{
    editor::{EditorManager, preview::HasPreview},
    grid::{GRID_SIZE, GridFootprint, RotatedFootprint, RotationIncrement, TileType},
};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct EditorGridGizmos;

pub fn setup_editor_gizmos(mut gizmo_config: ResMut<GizmoConfigStore>) {
    let (config, _) = gizmo_config.config_mut::<EditorGridGizmos>();
    config.line.width = 2.0;
}

fn draw_footprint(
    gizmos: &mut Gizmos<EditorGridGizmos>,
    world_pos: Vec2,
    footprint: RotatedFootprint,
) {
    let offsets = vec![ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

    for grid in footprint.with_position(IVec2::ZERO).grids() {
        let color = match grid.tile() {
            TileType::Hull => DARK_CYAN,
            TileType::Unobtrusive => GRAY,
            TileType::Interior => YELLOW,
            TileType::Exterior => GREEN,
            TileType::SemiExclusive => ORANGE,
            TileType::Exclusive => DARK_RED,
        };

        for tile_pos in grid.positions() {
            let tile_center = world_pos - footprint.offset() + tile_pos.as_vec2() * GRID_SIZE;

            for offset in &offsets {
                if grid.contains(tile_pos + offset) {
                    continue;
                }

                let dist = GRID_SIZE / 2.0;
                let center =
                    tile_center + offset.as_vec2() * (dist - gizmos.config.line.width / 2.0);
                let perp = offset.yx().as_vec2() * dist;

                gizmos.line_2d(center + perp, center - perp, color);
            }
        }
    }
}

pub fn draw_grid_boxes(
    editors: Query<(&HasPreview, &GlobalTransform), With<EditorManager>>,
    grid_footprints: Query<(
        &RotationIncrement,
        &GridFootprint,
        &GlobalTransform,
        &InheritedVisibility,
    )>,
    mut gizmos: Gizmos<EditorGridGizmos>,
) {
    editors.iter().for_each(|(editor, editor_transform)| {
        if let Ok((rotation, footprint, transform, visibility)) =
            grid_footprints.get(editor.preview())
            && matches!(*visibility, InheritedVisibility::VISIBLE)
        {
            draw_footprint(
                &mut gizmos,
                transform.translation().truncate(),
                RotatedFootprint::new(footprint, *rotation),
            );
        }

        let Some((tiles, entities)) = editor.placement_errors() else {
            return;
        };

        grid_footprints.iter_many(entities).matched().for_each(
            |(rotation, footprint, transform, _)| {
                draw_footprint(
                    &mut gizmos,
                    transform.translation().truncate(),
                    RotatedFootprint::new(footprint, *rotation),
                );
            },
        );

        for tile_pos in tiles {
            let world_pos = editor_transform.translation().truncate();
            let tile_center = world_pos + tile_pos.as_vec2() * GRID_SIZE;
            let size = Vec2::splat(GRID_SIZE - gizmos.config.line.width);

            gizmos.rect_2d(Isometry2d::from_translation(tile_center), size, RED);
        }
    });
}
