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
    pub fn new() -> Self {
        let arena = Arena::default();
        let mut world = Self {
            entities: Vec::default(),
            coordinate: Vec::with_capacity(arena.size),
            velocity: Vec::with_capacity(arena.size),
            health: Vec::with_capacity(arena.size),
            glyph: Vec::with_capacity(arena.size),
            arena,
        };
        world.spawn_player();
        world
    }

    pub fn tick(&mut self) {
        self.sys_movement();
        self.sys_cleanup();
    }

    pub fn player(&self) -> Option<Entity> {
        self.entities
            .iter()
            .find(|e| e.kind == EntityKind::Player)
            .copied()
    }

    fn alloc_entity(&mut self, kind: EntityKind) -> Entity {
        let i = self.arena.alloc();
        let e = Entity::new(i, kind);
        self.entities.push(e);

        // extend component vectors if arena grew
        if i == self.coordinate.len() {
            self.coordinate.push(None);
            self.velocity.push(None);
            self.health.push(None);
            self.glyph.push(None);
        }
        e
    }

    fn free_entity(&mut self, e: Entity) {
        self.entities.retain(|ee| ee.id != e.id);
        self.coordinate[e.id] = None;
        self.velocity[e.id] = None;
        self.health[e.id] = None;
        self.glyph[e.id] = None;
        self.arena.free(e.id);
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
