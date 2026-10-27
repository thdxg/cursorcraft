use crate::world::entity::Entity;

use super::{World, entity::EntityKind};

impl World {
    /// Update entity coordinates
    pub(super) fn sys_movement(&mut self) {
        for e in &self.entities {
            if let (Some(vel), Some(coord)) =
                (self.velocity[e.id].as_mut(), self.coordinate[e.id].as_mut())
            {
                coord.step(*vel);
                // reset player velocity after one move
                if e.kind == EntityKind::Player {
                    vel.speed = 0.0;
                }
            }
        }
    }

    /// Free dead entities
    pub(super) fn sys_cleanup(&mut self) {
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
