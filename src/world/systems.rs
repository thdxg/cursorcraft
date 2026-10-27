use super::{World, entity::EntityKind};

impl World {
    pub(super) fn movement(&mut self) {
        self.entities.iter().for_each(|e| {
            if let (Some(vel), Some(coord)) =
                (self.velocity[e.id].as_mut(), self.coordinate[e.id].as_mut())
            {
                coord.step(*vel);
                // reset player velocity after one move
                if e.kind == EntityKind::Player {
                    vel.speed = 0.0;
                }
            }
        });
    }

    pub(super) fn zombie_ai(&mut self) {
        todo!();
    }
}
