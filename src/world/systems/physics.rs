use crate::world::{World, entity::EntityKind};

impl World {
    /// Physics system including movment and contact.
    pub(super) fn physics(&mut self) {
        self.update_occupancy();
        self.movement();
    }

    /// Update entity coordinates.
    fn movement(&mut self) {
        for e in &self.entities {
            if let (Some(vel), Some(coord)) =
                (self.velocity[e.id].as_mut(), self.coordinate[e.id].as_mut())
            {
                // check if the cell changes or target cell is already occupied
                let next_coord = coord.next(*vel);
                let occupier = self.occupancy.get(&next_coord.cell());
                if coord.cell() == next_coord.cell() || occupier.is_none() {
                    // cell free
                    self.occupancy.remove(&coord.cell());
                    self.occupancy.insert(next_coord.cell(), *e);
                    *coord = next_coord;
                } else if let Some(occupier) = occupier {
                    // cell occupied
                    if !e.kind.fights(occupier.kind) {
                        continue;
                    }
                    let (a, b) = if e.id < occupier.id {
                        (e.id, occupier.id)
                    } else {
                        (occupier.id, e.id)
                    };
                    let (health1, health2) = self.health.split_at_mut(b);
                    if let (Some(ha), Some(hb)) =
                        (health1[a].as_mut(), health2[0].as_mut())
                    {
                        ha.trade(hb);
                    }
                }

                // reset player velocity after one move
                if e.kind == EntityKind::Player {
                    vel.speed = 0.0;
                }
            }
        }
    }

    fn update_occupancy(&mut self) {
        self.occupancy.clear();
        for e in &self.entities {
            if let Some(coord) = self.coordinate[e.id] {
                self.occupancy.insert(coord.cell(), *e);
            }
        }
    }
}
