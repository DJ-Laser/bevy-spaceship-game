use bevy::prelude::*;

use crate::{
    grid::{
        FootprintValidateError, GRID_SIZE, GridFootprint, GridManager, PositionedFootprint,
        RotationIncrement,
    },
    ship::part_components::InactivePart,
};

#[derive(Component, Clone, FromTemplate)]
#[require(Visibility::Hidden, InactivePart, Transform)]
#[relationship(relationship_target = HasPreview)]
pub struct EditorPreview(pub Entity);

#[derive(Component, Clone)]
#[relationship_target(relationship = EditorPreview, linked_spawn)]
pub struct HasPreview {
    #[relationship]
    preview_entity: Entity,
    cursor_position: Option<Vec2>,
    cursor_grid_position: Option<IVec2>,
    preview_translation: Option<Vec2>,
    placement_errors: Option<FootprintValidateError>,
}

impl HasPreview {
    pub fn preview(&self) -> Entity {
        self.preview_entity
    }

    pub fn set_cursor_none(&mut self) {
        self.cursor_position = None;
        self.cursor_grid_position = None;
        self.preview_translation = None;
        self.placement_errors = None;
    }

    pub fn update_preview(
        &mut self,
        grid: &GridManager,
        footprint_rot: RotationIncrement,
        preview_footprint: &GridFootprint,
    ) {
        let Some(grid_pos) = self.cursor_grid_position else {
            return;
        };

        let errors = grid.validte_footprint(PositionedFootprint::new(
            preview_footprint,
            grid_pos,
            footprint_rot,
        ));
        self.placement_errors = errors.err();
    }

    pub fn set_cursor(
        &mut self,
        cursor: Vec2,
        grid: &GridManager,
        footprint_rot: RotationIncrement,
        preview_footprint: &GridFootprint,
    ) {
        self.cursor_position = Some(cursor);

        let prev_grid_pos = self.cursor_grid_position;
        let grid_pos = ((cursor - preview_footprint.offset) / GRID_SIZE)
            .round()
            .as_ivec2();

        self.cursor_grid_position = Some(grid_pos);
        if self.cursor_grid_position != prev_grid_pos {
            self.update_preview(grid, footprint_rot, preview_footprint);
        }

        let preview_translation = grid_pos.as_vec2() * GRID_SIZE + preview_footprint.offset;
        self.preview_translation = Some(preview_translation);
    }

    pub fn move_cursor(
        &mut self,
        cursor_offset: Vec2,
        grid: &GridManager,
        footprint_rot: RotationIncrement,
        preview_footprint: &GridFootprint,
    ) {
        let cursor = self.cursor_position.unwrap_or(Vec2::ZERO);

        self.set_cursor(
            cursor + cursor_offset,
            grid,
            footprint_rot,
            preview_footprint,
        );
    }

    pub fn cursor_grid_position(&self) -> Option<IVec2> {
        self.cursor_grid_position
    }

    pub fn preview_translation(&self) -> Option<Vec2> {
        self.preview_translation
    }

    pub fn placement_errors(&self) -> &Option<FootprintValidateError> {
        &self.placement_errors
    }
}
