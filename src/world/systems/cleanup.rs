use crate::world::{
    World,
    entity::{Entity, EntityKind},
};

impl World {
    /// Cleanup system freeing dead entities.
    pub(super) fn cleanup(&mut self) {
        let to_free: Vec<Entity> = self
            .entities
            .iter()
            .filter(|e| self.health[e.id].is_some_and(|h| !h.is_alive()))
            .copied()
            .collect();

        for e in to_free {
            if e.kind == EntityKind::Zombie {
                self.stats.zombies_killed += 1;
            }
            self.free_entity(e);
        }
    }
}
