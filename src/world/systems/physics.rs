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
                let next_coord = coord.step(*vel);
                // check every cell between coord and next_coord
                let mut is_clear = true;
                for cell in coord.cells_to(next_coord) {
                    // check if cell is occupied
                    let Some(occupier) = self.occupancy.get(&cell) else {
                        continue;
                    };
                    if e.kind.penetrates(occupier.kind) {
                        continue;
                    }

                    if occupier.kind.absorbs(e.kind) {
                        let health = self.health[e.id].as_mut().unwrap();
                        health.kill();
                    } else if occupier.kind.fights(e.kind) {
                        // a: lower id, b: higher id
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
                    let is_alive = |id: usize| {
                        self.health[id].is_some_and(|h| h.is_alive())
                    };
                    is_clear &= is_alive(e.id) && !is_alive(occupier.id);
                    if !is_clear {
                        break;
                    }
                }

                if is_clear {
                    if self.occupancy.get(&coord.cell()) == Some(e) {
                        self.occupancy.remove(&coord.cell());
                    }
                    self.occupancy.insert(next_coord.cell(), *e);
                    *coord = next_coord;
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
