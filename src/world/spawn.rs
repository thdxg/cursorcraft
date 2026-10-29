use super::{
    World,
    components::{Coordinate, Direction, Health, Velocity},
    entity::{Entity, EntityKind},
};

impl World {
    pub fn spawn_player(&mut self) -> Entity {
        let e = self.alloc_entity(EntityKind::Player);
        self.coordinate[e.id] = Some(Coordinate::new(0.0, 0.0));
        self.velocity[e.id] = Some(Velocity::new(0.0, Direction::Up));
        self.health[e.id] = Some(Health::new(100.0, 100.0));
        self.glyph[e.id] = None;
        e
    }

    pub fn spawn_zombie(
        &mut self,
        coordinate: Coordinate,
        health: Health,
    ) -> Entity {
        let e = self.alloc_entity(EntityKind::Zombie);
        self.coordinate[e.id] = Some(coordinate);
        self.velocity[e.id] = Some(Velocity::new(0.0, Direction::Up));
        self.health[e.id] = Some(health);
        self.glyph[e.id] = Some(' ');
        e
    }

    pub fn spawn_block(&mut self, coordinate: Coordinate) -> Entity {
        let e = self.alloc_entity(EntityKind::Block);
        self.coordinate[e.id] = Some(coordinate);
        self.velocity[e.id] = None;
        self.health[e.id] = Some(Health::new(20.0, 20.0));
        self.glyph[e.id] = Some('x');
        e
    }
}
