use super::{
    World,
    components::{Coordinate, Direction, Health, Velocity},
    entity::{Entity, EntityKind},
};

impl World {
    pub fn spawn_player(&mut self) -> Entity {
        let i = self.arena.alloc();
        self.coordinate[i] = Some(Coordinate::new(0.0, 0.0));
        self.velocity[i] = Some(Velocity::new(0.0, Direction::Up));
        self.health[i] = Some(Health::new(100.0, 100.0));
        self.glyph[i] = None;

        let e = Entity::new(i, EntityKind::Player);
        self.entities.push(e);
        e
    }

    pub fn spawn_zombie(
        &mut self,
        coordinate: Coordinate,
        health: Health,
    ) -> Entity {
        let i = self.arena.alloc();
        self.coordinate[i] = Some(coordinate);
        self.velocity[i] = Some(Velocity::new(1.0, Direction::Up));
        self.health[i] = Some(health);
        self.glyph[i] = Some('!');

        let e = Entity::new(i, EntityKind::Zombie);
        self.entities.push(e);
        e
    }

    pub fn spawn_block(&mut self, coordinate: Coordinate) -> Entity {
        let i = self.arena.alloc();
        self.coordinate[i] = Some(coordinate);
        self.velocity[i] = None;
        self.health[i] = Some(Health::new(20.0, 20.0));
        self.glyph[i] = Some('x');

        let e = Entity::new(i, EntityKind::Block);
        self.entities.push(e);
        e
    }
}
