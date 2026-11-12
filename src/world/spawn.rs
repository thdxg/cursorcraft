use crate::world::{
    World,
    components::{Coordinate, Direction, Health, Velocity},
    entity::{Entity, EntityKind},
};

const PLAYER_HEALTH: f64 = 100.0;
const BLOCK_HEALTH: f64 = 20.0;
const BLOCK_GLYPH: char = 'x';
const BULLET_HEALTH: f64 = 20.0;
const BULLET_SPEED: f64 = 2.0;
const BULLET_GLYPH_HOR: char = '-';
const BULLET_GLYPH_VER: char = '∣';

impl World {
    pub fn spawn_player(&mut self) -> Entity {
        let e = self.alloc_entity(EntityKind::Player);
        self.coordinate[e.id] = Some(Coordinate::new(0.0, 0.0));
        self.velocity[e.id] = Some(Velocity::new(0.0, Direction::Up));
        self.health[e.id] = Some(Health::new(PLAYER_HEALTH, PLAYER_HEALTH));
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
        self.health[e.id] = Some(Health::new(BLOCK_HEALTH, BLOCK_HEALTH));
        self.glyph[e.id] = Some(BLOCK_GLYPH);
        e
    }

    pub fn spawn_bullet(
        &mut self,
        coordinate: Coordinate,
        dir: Direction,
    ) -> Entity {
        let e = self.alloc_entity(EntityKind::Bullet);
        self.coordinate[e.id] = Some(coordinate);
        self.velocity[e.id] = Some(Velocity::new(BULLET_SPEED, dir));
        self.health[e.id] = Some(Health::new(BULLET_HEALTH, BULLET_HEALTH));
        self.glyph[e.id] = Some(match dir {
            Direction::Left | Direction::Right => BULLET_GLYPH_HOR,
            Direction::Up | Direction::Down => BULLET_GLYPH_VER,
        });
        e
    }
}
