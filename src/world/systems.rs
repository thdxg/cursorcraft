mod cleanup;
mod physics;
mod zombie;

use crate::world::World;

impl World {
    /// Runs a cycle of systems
    pub fn tick(&mut self) {
        self.physics();
        self.zombie();
        self.cleanup();
    }
}
