use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

use crate::grid::{RotationIncrement, RotationStyle, TileType};

pub struct GridFootprintBuilder(HashMap<TileType, HashSet<IVec2>>);

impl GridFootprintBuilder {
    fn positions_mut(&mut self, tile: TileType) -> &mut HashSet<IVec2> {
        self.0.entry(tile).or_default()
    }

    pub fn tile(mut self, tile: TileType, pos: IVec2) -> Self {
        self.positions_mut(tile).insert(pos);

        self
    }

    pub fn rect(mut self, tile: TileType, a: IVec2, b: IVec2) -> Self {
        let positions = self.positions_mut(tile);

        for x in a.x..=b.x {
            for y in a.y..=b.y {
                positions.insert(ivec2(x, y));
            }
        }

        self
    }

    pub fn build_offset(self, offset: Vec2) -> GridFootprint {
        GridFootprint {
            tiles: self.0,
            offset,
        }
    }

    pub fn build(self) -> GridFootprint {
        self.build_offset(Vec2::ZERO)
    }
}

/// Defines an object's grid footprint
/// Coordinates are relative to the object's origin
#[derive(Component, Default, Clone)]
#[require(Visibility, Transform, RotationStyle)]
pub struct GridFootprint {
    pub tiles: HashMap<TileType, HashSet<IVec2>>,
    offset: Vec2,
}

impl GridFootprint {
    pub fn builder() -> GridFootprintBuilder {
        GridFootprintBuilder(Default::default())
    }
}

#[derive(Clone, Copy)]
pub struct RotatedFootprint<'a> {
    footprint: &'a GridFootprint,
    rotation: RotationIncrement,
}

impl<'a> RotatedFootprint<'a> {
    pub fn new(footprint: &'a GridFootprint, rotation: RotationIncrement) -> Self {
        Self {
            footprint,
            rotation,
        }
    }

    pub fn with_position(self, position: IVec2) -> PositionedFootprint<'a> {
        PositionedFootprint::new(self.footprint, position, self.rotation)
    }

    pub fn offset(&self) -> Vec2 {
        self.footprint
            .offset
            .rotate_angle(self.rotation.rotation_radians())
    }
}

#[derive(Clone, Copy)]
pub struct PositionedFootprint<'a> {
    footprint: &'a GridFootprint,
    position: IVec2,
    rotation: RotationIncrement,
}

impl<'a> PositionedFootprint<'a> {
    pub fn new(footprint: &'a GridFootprint, position: IVec2, rotation: RotationIncrement) -> Self {
        Self {
            footprint,
            position,
            rotation,
        }
    }

    pub fn offset(&self) -> Vec2 {
        self.footprint
            .offset
            .rotate_angle(self.rotation.rotation_radians())
    }

    pub fn tiles(&self) -> impl Iterator<Item = (TileType, IVec2)> {
        self.footprint.tiles.iter().flat_map(|(tile, offsets)| {
            offsets.iter().map(|offset| {
                (
                    *tile,
                    self.position + self.rotation.rotate_tile_pos(*offset),
                )
            })
        })
    }

    pub fn grids(&self) -> impl Iterator<Item = PositionedFootprintTiles<'_>> {
        self.footprint
            .tiles
            .iter()
            .map(|(tile, grid)| PositionedFootprintTiles {
                tile: *tile,
                grid,
                position: self.position,
                rotation: self.rotation,
            })
    }
}

pub struct PositionedFootprintTiles<'a> {
    tile: TileType,
    grid: &'a HashSet<IVec2>,
    position: IVec2,
    rotation: RotationIncrement,
}

impl<'a> PositionedFootprintTiles<'a> {
    pub fn tile(&self) -> TileType {
        self.tile
    }

    pub fn contains(&self, pos: IVec2) -> bool {
        let rotated_offset = pos - self.position;
        let offset = self.rotation.inverse().rotate_tile_pos(rotated_offset);
        self.grid.contains(&offset)
    }

    pub fn positions(&self) -> impl Iterator<Item = IVec2> {
        self.grid
            .iter()
            .map(|offset| self.position + self.rotation.rotate_tile_pos(*offset))
    }
}
