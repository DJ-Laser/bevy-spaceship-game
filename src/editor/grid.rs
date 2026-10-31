use avian2d::parry::utils::{hashmap::HashMap, hashset::HashSet};
use bevy::{
    color::palettes::css::{DARK_CYAN, GRAY, GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

pub const GRID_SIZE: f32 = 20.0;

pub type FootprintValidateError = (HashSet<IVec2>, HashSet<Entity>);

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

impl TileType {
    fn can_place_without_hull(&self) -> bool {
        matches!(
            self,
            TileType::Hull | TileType::SemiExclusive | TileType::Exclusive
        )
    }
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
    pub offset: Vec2,
}

impl GridFootprint {
    pub fn builder() -> GridFootprintBuilder {
        GridFootprintBuilder(Default::default())
    }
}

#[derive(Component, Default, Clone)]
pub struct GridManager {
    tiles: HashMap<IVec2, HashMap<TileType, Entity>>,
}

impl GridManager {
    fn get_tiles_at(&self, pos: IVec2) -> Option<&HashMap<TileType, Entity>> {
        self.tiles.get(&pos)
    }

    fn get_tiles_at_mut(&mut self, pos: IVec2) -> &mut HashMap<TileType, Entity> {
        self.tiles.entry(pos).or_default()
    }

    fn can_place_tile_at(&self, pos: IVec2, tile: TileType) -> Result<(), Vec<Entity>> {
        let Some(tiles) = self.get_tiles_at(pos) else {
            return tile
                .can_place_without_hull()
                .then_some(())
                .ok_or_else(Vec::new);
        };

        if let Some(entity) = tiles.get(&TileType::Exclusive) {
            return Err(vec![*entity]);
        }

        if !matches!(tile, TileType::Hull)
            && let Some(entity) = tiles.get(&TileType::SemiExclusive)
        {
            return Err(vec![*entity]);
        }

        let no_hull_err = (!tiles.contains_key(&TileType::Hull)).then(Vec::new);

        fn one<T: Copy>(item: &T) -> Vec<T> {
            vec![*item]
        }

        let errors = match tile {
            TileType::Hull => tiles.get(&TileType::Hull).map(one),
            TileType::Unobtrusive => no_hull_err,
            tile @ (TileType::Interior | TileType::Exterior) => {
                tiles.get(&tile).map(one).or(no_hull_err)
            }
            TileType::SemiExclusive => {
                let entities: Vec<_> = tiles
                    .iter()
                    .filter(|(tile, _)| !matches!(tile, TileType::Hull))
                    .map(|(_, entity)| entity)
                    .copied()
                    .collect();
                (!entities.is_empty()).then_some(entities).or(no_hull_err)
            }

            TileType::Exclusive => {
                let entities: Vec<_> = tiles.values().copied().collect();
                (!entities.is_empty()).then_some(entities)
            }
        };

        if let Some(errors) = errors {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn place_tile_at(&mut self, pos: IVec2, tile: TileType, entity: Entity) {
        let tiles = self.get_tiles_at_mut(pos);
        tiles.insert(tile, entity);
    }

    /// Checks if the footprint can be placed at a certain position on the grid
    /// If placement fails, returns a list of colliding entities
    pub fn validte_footprint(
        &self,
        footprint_pos: IVec2,
        footprint: &GridFootprint,
    ) -> Result<(), FootprintValidateError> {
        let mut res = Ok(());

        for (tile, offsets) in &footprint.tiles {
            for offset in offsets {
                let grid_coord = footprint_pos + offset;
                if let Err(tile_errors) = self.can_place_tile_at(grid_coord, *tile) {
                    let (mut positions, mut errors): FootprintValidateError =
                        res.err().unwrap_or_default();
                    positions.insert(grid_coord);
                    errors.extend(tile_errors);
                    res = Err((positions, errors));
                }
            }
        }

        res
    }

    pub fn place_footprint(&mut self, footprint_pos: IVec2, footprint: (Entity, &GridFootprint)) {
        for (tile, offsets) in &footprint.1.tiles {
            for offset in offsets {
                self.place_tile_at(footprint_pos + offset, *tile, footprint.0);
            }
        }
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

            for (tile, offsets) in &footprint.tiles {
                for tile_offset in offsets {
                    gizmos.rect_2d(
                        Isometry2d::new(
                            transform.translation().truncate() + tile_offset.as_vec2() * GRID_SIZE
                                - footprint.offset,
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
                }
            }
        });
}
