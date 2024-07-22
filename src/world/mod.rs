use conv::{ApproxFrom, ApproxInto};
use std::mem::swap;
use std::ops::Deref;

use crate::vec2d::Vec2d;
use slab::Slab;

use crate::world::entity::{Entity, EntityTrait};
use crate::world::tile::Tile;

pub mod entity;
pub mod player;
pub mod render;
pub mod tile;

#[derive(Debug)]
pub struct World {
    tiles: [[Tile; World::WIDTH]; World::HEIGHT],
    entities: Slab<Entity>,
}

impl World {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 10;

    pub fn from_tiles(tiles: [[Tile; World::WIDTH]; World::HEIGHT]) -> Self {
        Self {
            tiles,
            entities: Default::default(),
        }
    }

    pub fn tile_vec(&self, vec: Vec2d) -> Option<&Tile> {
        let coords = <Vec2d as ApproxInto<[usize; 2]>>::approx_into(vec).ok()?;
        self.tile(coords[0], coords[1])
    }
    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(x).and_then(|tiles| tiles.get(y))
    }

    pub fn set_tile(&mut self, x: usize, y: usize, mut new_tile: Tile) -> Option<Tile> {
        let tile_at = self.tiles.get_mut(x).and_then(|tiles| tiles.get_mut(y))?;
        swap(tile_at, &mut new_tile);
        Some(new_tile)
    }

    pub fn entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(*id)
    }
    pub fn entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(*id)
    }
    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        EntityId(self.entities.insert(entity))
    }
    pub fn remove_entity(&mut self, id: EntityId) -> Entity {
        self.entities.remove(*id)
    }

    pub fn update(&mut self) {
        for (_, entity) in self.entities.iter_mut() {
            entity.update();
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct EntityId(usize);

impl Deref for EntityId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            tiles: [[Tile::default(); Self::WIDTH]; Self::HEIGHT],
            entities: Slab::default(),
        }
    }
}
