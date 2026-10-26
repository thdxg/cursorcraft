use std::any::Any;

use super::{Entity, Game, Hp};
use crate::game::geometry::{Coord, Dir};

const BULLET_VELOCITY: f64 = 0.3;
const BULLET_RANGE: f64 = 20.0;

#[derive(Copy, Clone)]
pub struct Bullet {
    coord: Coord,
    hp: Hp,
    dir: Dir,
    traveled: f64,
}

impl Bullet {
    pub fn new(coord: Coord, hp: Hp, dir: Dir) -> Self {
        Self {
            coord,
            hp,
            dir,
            traveled: 0.0,
        }
    }

    /// Detects contact with the player and subtracts.
    fn update_hp(&mut self, game: &Game) {
        if self.coord.is_near(game.player.coord()) {
            self.hp -= game.player.hp();
        }
    }

    fn step(&mut self) {
        self.coord.step(BULLET_VELOCITY, self.dir);
        self.traveled += BULLET_VELOCITY;
    }
}

impl Entity for Bullet {
    fn coord(&self) -> Coord {
        self.coord
    }

    /// Hp decreases based on distance traveled.
    fn hp(&self) -> Hp {
        let power = ((BULLET_RANGE - self.traveled) / BULLET_RANGE).max(0.0);
        self.hp * power
    }

    fn tick(&self, game: &mut Game) -> Option<Self> {
        let mut next = *self;
        // check hp before update, so reaching zero hp after update_hp returns
        // Some one last time
        if !next.hp.is_alive() || next.traveled >= BULLET_RANGE {
            return None;
        }
        next.update_hp(game);
        next.step();
        Some(next)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
