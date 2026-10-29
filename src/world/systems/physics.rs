use std::collections::HashMap;

use crate::world::{
    World,
    components::Coordinate,
    entity::{Entity, EntityKind},
};

impl World {
    /// Physics system including movment and contact.
    pub(super) fn physics(&mut self) {
        self.movement();
    }

    /// Update entity coordinates.
    fn movement(&mut self) {
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

    fn collision(&mut self) -> bool {
        // construct map
        let mut map: HashMap<Coordinate, Vec<Entity>> = HashMap::new();
        for e in &self.entities {
            let Some(coord) = self.coordinate[e.id] else {
                continue;
            };
            map.entry(coord).or_default().push(*e);
        }
        false
    }
}
