use crate::world::{World, entity::Entity};

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
            self.free_entity(e);
        }
    }
}
