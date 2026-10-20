use super::{Entity, Game, Hp};
use crate::game::geometry::{Dir, Pos};

#[derive(Copy, Clone)]
pub struct Player {
    pos: Pos,
    hp: Hp,
    velocity: f64,
}

impl Player {
    pub fn new(pos: Pos, hp: Hp, velocity: f64) -> Self {
        Self { pos, hp, velocity }
    }

    pub fn update_pos(&mut self, dir: Dir) {
        self.pos.step(self.velocity, dir);
    }
}

impl Entity for Player {
    fn pos(&self) -> Pos {
        self.pos
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, game: &Game) -> Option<Self> {
        let mut next = *self;
        let total_dmg: Hp =
            game.zombies.iter().fold(Hp::default(), |acc, zombie| {
                if self.pos.is_near(&zombie.pos()) {
                    acc + zombie.hp()
                } else {
                    acc
                }
            });
        next.hp -= total_dmg;
        if !next.hp.is_alive() {
            return None;
        }
        Some(next)
    }
}
