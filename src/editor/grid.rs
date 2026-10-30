use avian2d::parry::utils::{hashmap::HashMap, hashset::HashSet};
use bevy::{
    color::palettes::css::{DARK_CYAN, GRAY, GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

pub const GRID_SIZE: f32 = 20.0;

pub fn snap_cursor(cursor: Vec2) -> Vec2 {
    (cursor / GRID_SIZE).round() * GRID_SIZE
}

/// Defines the tile layers for the ship editor
/// Generally, tiles need a hull tile in the same region to be placed, and can't overlap
/// Exclusive tiles are the exception, and require the absence of hull tiles to be placed
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    /// Defines the ship's hull and allows other typrs of tiles to be placed atop it
    /// Hull tiles may not overlap
    Hull,
    /// Unobtrusive tiles require a hull to be placed and CAN overlap
    Unobtrusive,
    /// Interior tiles are placed "inside" the hull and can't overlap.
    Interior,
    /// Exterior tiles are placed "atop" the hull and can't overlap.
    Exterior,
    /// SemiExclusive tiles may or may not have a hull, but can't overlap with any other tile
    SemiExclusive,
    /// Exclusive tiles require a hull to NOT be placed and can't overlap
    Exclusive,
}

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
#[require(Visibility, Transform)]
pub struct GridFootprint {
    tiles: HashMap<TileType, HashSet<IVec2>>,
    offset: Vec2,
}

impl GridFootprint {
    pub fn builder() -> GridFootprintBuilder {
        GridFootprintBuilder(Default::default())
    }

    fn visit_all(&self, mut visitor: impl FnMut(TileType, IVec2)) {
        for (tile, offsets) in &self.tiles {
            for offset in offsets {
                visitor(*tile, *offset)
            }
        }
    }

    pub fn snap_with_offset(&self, cursor: Vec2) -> Vec2 {
        snap_cursor(cursor + self.offset) - self.offset
    }

    pub fn translation_to_grid(&self, translation: Vec2) -> IVec2 {
        ((translation + self.offset) / GRID_SIZE).as_ivec2()
    }
}

#[derive(Component, Default, Clone)]
pub struct GridManager {
    tiles: HashMap<IVec2, HashSet<TileType>>,
}

impl GridManager {
    fn get_tiles_at(&self, pos: IVec2) -> Option<&HashSet<TileType>> {
        self.tiles.get(&pos)
    }

    fn get_tiles_at_mut(&mut self, pos: IVec2) -> &mut HashSet<TileType> {
        self.tiles.entry(pos).or_default()
    }

    fn can_place_tile_at(&self, pos: IVec2, tile: TileType) -> bool {
        let Some(tiles) = self.get_tiles_at(pos) else {
            return matches!(
                tile,
                TileType::Hull | TileType::SemiExclusive | TileType::Exclusive
            );
        };

        if tiles.contains(&TileType::Exclusive)
            || (!matches!(tile, TileType::Hull) && tiles.contains(&TileType::SemiExclusive))
        {
            return false;
        }

        match tile {
            TileType::Hull => !tiles.contains(&TileType::Hull),
            TileType::Unobtrusive => tiles.contains(&TileType::Hull),
            tile @ (TileType::Interior | TileType::Exterior) => {
                tiles.contains(&TileType::Hull) && !tiles.contains(&tile)
            }
            TileType::SemiExclusive => {
                tiles.is_empty() || (tiles.contains(&TileType::Hull) && tiles.len() == 1)
            }
            TileType::Exclusive => tiles.is_empty(),
        }
    }

    fn place_tile_at(&mut self, pos: IVec2, tile: TileType) {
        let tiles = self.get_tiles_at_mut(pos);
        tiles.insert(tile);
    }

    pub fn validte_footprint(&self, footprint_pos: IVec2, footprint: &GridFootprint) -> bool {
        for (tile, offsets) in &footprint.tiles {
            for offset in offsets {
                if !self.can_place_tile_at(footprint_pos + offset, *tile) {
                    return false;
                }
            }
        }

        true
    }

    pub fn place_footprint(&mut self, footprint_pos: IVec2, footprint: &GridFootprint) {
        footprint.visit_all(|tile, offset| {
            self.place_tile_at(footprint_pos + offset, tile);
        });
    }
}

pub fn draw_grid_boxes(
    grid_footprints: Query<(&GridFootprint, &GlobalTransform, &InheritedVisibility)>,
    mut gizmos: Gizmos,
) {
    grid_footprints
        .iter()
        .for_each(|(footprint, transform, visibility)| {
            if matches!(*visibility, InheritedVisibility::HIDDEN) {
                return;
            }

            footprint.visit_all(|tile, tile_offset| {
                gizmos.rect_2d(
                    Isometry2d::new(
                        transform.translation().truncate()
                            + tile_offset.as_vec2() * GRID_SIZE
                            + footprint.offset,
                        Rot2::default(),
                    ),
                    Vec2::splat(GRID_SIZE),
                    match tile {
                        TileType::Hull => DARK_CYAN,
                        TileType::Unobtrusive => GRAY,
                        TileType::Interior => YELLOW,
                        TileType::Exterior => GREEN,
                        TileType::SemiExclusive => ORANGE,
                        TileType::Exclusive => RED,
                    },
                );
            });
        });
}
