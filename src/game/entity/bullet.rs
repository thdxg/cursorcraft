const BULLET_VELOCITY: f64 = 0.1;

use super::{Entity, Game, Hp};
use crate::game::geometry::{Dir, Pos};

#[derive(Copy, Clone)]
pub struct Bullet {
    pub pos: Pos,
    pub hp: Hp,
    pub dir: Dir,
}

impl Bullet {
    pub fn new(pos: Pos, hp: Hp, dir: Dir) -> Self {
        Self { pos, hp, dir }
    }
}

impl Entity for Bullet {
    fn pos(&self) -> Pos {
        self.pos
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, _: &Game) -> Option<Self> {
        let mut next = *self;
        next.pos.step(BULLET_VELOCITY, self.dir);
        Some(next)
    }
}
