const ZOMBIE_GLYPH_UPDATE_FREQ: u16 = 5; // ticks

use super::{Entity, Game, Hp, Player};
use crate::game::{
    geometry::{Dir, Pos},
    glyph,
};

#[derive(Copy, Clone)]
pub struct Zombie {
    pos: Pos,
    hp: Hp,
    pub dir: Dir,
    pub velocity: f64,
    pub glyph: char,
    ticks_since_glyph_update: u16,
}

impl Zombie {
    pub fn new(pos: Pos, hp: Hp, dir: Dir, velocity: f64) -> Self {
        let mut enemy = Self {
            pos,
            hp,
            dir,
            velocity,
            glyph: ' ',
            ticks_since_glyph_update: 0,
        };
        enemy.update_glyph();
        enemy
    }

    fn update_glyph(&mut self) {
        if self.ticks_since_glyph_update < ZOMBIE_GLYPH_UPDATE_FREQ {
            self.ticks_since_glyph_update += 1;
            return;
        }
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "clamped to 0.0..=8.0"
        )]
        let dots = (self.hp.ratio() * 8.0) as usize;
        self.glyph = glyph::braille_with_count(dots);
        self.ticks_since_glyph_update = 0;
    }

    fn update_dir(&mut self, player: &Player) {
        self.dir = self.pos.dir_to(&player.pos());
    }

    fn update_pos(&mut self) {
        self.pos.step(self.velocity, self.dir);
    }
}

impl Entity for Zombie {
    fn pos(&self) -> Pos {
        self.pos
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, game: &Game) -> Option<Self> {
        let mut next = *self;
        next.update_glyph();
        if self.pos.is_near(&game.player.pos()) {
            next.hp -= game.player.hp();
        }
        if !next.hp.is_alive() {
            return None;
        }
        next.update_dir(&game.player);
        next.update_pos();
        Some(next)
    }
}
