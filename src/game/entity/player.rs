use super::{Entity, Game, Hp};
use crate::game::geometry::{Coord, Dir};

#[derive(Copy, Clone)]
pub struct Player {
    coord: Coord,
    hp: Hp,
    velocity: f64,
}

impl Player {
    pub fn new(coord: Coord, hp: Hp, velocity: f64) -> Self {
        Self {
            coord,
            hp,
            velocity,
        }
    }

    pub fn step(&mut self, dir: Dir) {
        self.coord.step(self.velocity, dir);
    }

    fn update_hp(&mut self, game: &Game) {
        let total_dmg: Hp = game.zombies.iter().fold(
            Hp::new(0.0, f64::INFINITY),
            |acc, zombie| {
                if self.coord.is_near(zombie.coord()) {
                    acc + zombie.hp()
                } else {
                    acc
                }
            },
        );
        self.hp -= total_dmg;
    }
}

impl Entity for Player {
    fn coord(&self) -> Coord {
        self.coord
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, game: &Game) -> Option<Self> {
        let mut next = *self;
        // check hp before update, so reaching zero hp after update_hp returns
        // Some one last time
        if !next.hp.is_alive() {
            return None;
        }
        next.update_hp(game);
        Some(next)
    }
}
