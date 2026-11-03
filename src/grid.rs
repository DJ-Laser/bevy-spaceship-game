mod footprint;
mod rotation;

use bevy::{
    ecs::entity::{EntityHashSet, MapEntities},
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

pub use footprint::{GridFootprint, PositionedFootprint, RotatedFootprint};
pub use rotation::{RotationIncrement, RotationStyle};

pub const GRID_SIZE: f32 = 20.0;

#[derive(Debug, Clone, Default)]
pub struct FootprintValidateError(pub HashSet<IVec2>, pub EntityHashSet);

impl MapEntities for FootprintValidateError {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        self.1.map_entities(entity_mapper)
    }
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

impl TileType {
    fn can_place_without_hull(&self) -> bool {
        matches!(
            self,
            TileType::Hull | TileType::SemiExclusive | TileType::Exclusive
        )
    }
}

#[derive(Component, Default, Clone)]
#[component(map_entities)]
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
        footprint: PositionedFootprint,
    ) -> Result<(), FootprintValidateError> {
        let mut res = Ok(());

        for (tile, tile_pos) in footprint.tiles() {
            if let Err(tile_errors) = self.can_place_tile_at(tile_pos, tile) {
                let FootprintValidateError(mut positions, mut errors) =
                    res.err().unwrap_or_default();
                positions.insert(tile_pos);
                errors.extend(tile_errors);
                res = Err(FootprintValidateError(positions, errors));
            }
        }

        res
    }

    pub fn place_footprint(&mut self, footprint_id: Entity, footprint: PositionedFootprint) {
        for (tile, tile_pos) in footprint.tiles() {
            self.place_tile_at(tile_pos, tile, footprint_id);
        }
    }
}

impl MapEntities for GridManager {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for tile_set in self.tiles.values_mut() {
            for entity in tile_set.values_mut() {
                *entity = entity_mapper.get_mapped(*entity);
            }
        }
    }
}
