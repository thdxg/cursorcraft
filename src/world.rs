pub mod components;
mod entity;
mod spawn;
mod systems;

use std::fmt::Debug;

use crate::world::{
    components::{Coordinate, Health, Velocity},
    entity::{Arena, Entity, EntityKind},
};

pub struct World {
    pub entities: Vec<Entity>,
    pub coordinate: Vec<Option<Coordinate>>,
    pub velocity: Vec<Option<Velocity>>,
    pub health: Vec<Option<Health>>,
    pub glyph: Vec<Option<char>>,
    arena: Arena,
}

impl World {
    pub fn tick(&mut self) {
        self.movement();
    }

    pub fn player(&self) -> Option<Entity> {
        self.entities
            .iter()
            .find(|e| e.kind == EntityKind::Player)
            .copied()
    }
}

impl Default for World {
    fn default() -> Self {
        let arena = Arena::default();
        Self {
            entities: Vec::default(),
            coordinate: vec![None; arena.size],
            velocity: vec![None; arena.size],
            health: vec![None; arena.size],
            glyph: vec![None; arena.size],
            arena,
        }
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("entities", &self.entities.len())
            .field("coordinate", &self.coordinate.len())
            .field("velocity", &self.velocity.len())
            .field("health", &self.health.len())
            .field("glyph", &self.glyph.len())
            .field("arena", &self.arena)
            .finish()
    }
}
