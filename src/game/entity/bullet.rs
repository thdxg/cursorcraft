const BULLET_VELOCITY: f64 = 0.1;

use super::{Entity, Game, Hp};
use crate::game::geometry::{Coord, Dir};

#[derive(Copy, Clone)]
pub struct Bullet {
    coord: Coord,
    hp: Hp,
    dir: Dir,
}

impl Bullet {
    pub fn new(coord: Coord, hp: Hp, dir: Dir) -> Self {
        Self { coord, hp, dir }
    }
}

impl Entity for Bullet {
    fn coord(&self) -> Coord {
        self.coord
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, _: &Game) -> Option<Self> {
        let mut next = *self;
        next.coord.step(BULLET_VELOCITY, self.dir);
        Some(next)
    }
}
