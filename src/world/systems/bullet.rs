use crate::world::{World, entity::EntityKind};

impl World {
    pub(super) fn bullet(&mut self) {
        self.decay_speed();
    }

    fn decay_speed(&mut self) {
        let bullets = self
            .entities
            .iter()
            .filter(move |e| e.kind == EntityKind::Bullet);
        for e in bullets {
            // decay speed
            let vel = self.velocity[e.id].as_mut().unwrap();
            vel.speed *= 0.98;
            if vel.speed < 0.5 {
                let health = self.health[e.id].as_mut().unwrap();
                health.kill();
            }
        }
    }
}
