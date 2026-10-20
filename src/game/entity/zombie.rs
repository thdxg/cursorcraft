const ZOMBIE_GLYPH_UPDATE_FREQ: u16 = 5; // ticks

use super::{Entity, Game, Hp, Player};
use crate::game::{
    geometry::{Coord, Dir},
    glyph,
};

#[derive(Copy, Clone)]
pub struct Zombie {
    coord: Coord,
    hp: Hp,
    pub dir: Dir,
    pub velocity: f64,
    pub glyph: char,
    ticks_since_glyph_update: u16,
}

impl Zombie {
    pub fn new(coord: Coord, hp: Hp, dir: Dir, velocity: f64) -> Self {
        let mut enemy = Self {
            coord,
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
        self.dir = self.coord.dir_to(player.coord());
    }

    fn step(&mut self) {
        self.coord.step(self.velocity, self.dir);
    }

    fn update_hp(&mut self, game: &Game) {
        if self.coord.is_near(game.player.coord()) {
            self.hp -= game.player.hp();
        }
    }
}

impl Entity for Zombie {
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
        next.update_glyph();
        next.update_hp(game);
        next.update_dir(&game.player);
        next.step();
        Some(next)
    }
}
