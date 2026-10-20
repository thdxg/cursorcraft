use std::ops::Range;

use super::{Entity, Game, Hp, Player};
use crate::game::{
    Bullet,
    geometry::{Coord, Dir},
    glyph,
};
use rand::{RngExt, seq::IndexedRandom};

const ZOMBIE_GLYPH_UPDATE_FREQ: u16 = 5; // ticks
const ZOMBIE_BULLET_HP: f64 = 10.0;
const INITIAL_HP_MIN: f64 = 5.0;
const INITIAL_HP_MAX: f64 = 20.0;
const INITIAL_VELOCITY_MIN: f64 = 0.01;
const INITIAL_VELOCITY_MAX: f64 = 0.03;
/// Available actions and probabilities
const ACTIONS: [(Action, f64); 2] =
    [(Action::Shoot, 0.001), (Action::Move, 0.999)];

#[derive(Copy, Clone)]
enum Action {
    Shoot,
    Move,
}

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
    pub fn spawn_in(area: Range<Coord>) -> Self {
        let mut rng = rand::rng();
        let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        Self {
            coord: Coord::new(
                rng.random_range(area.start.x..area.end.x),
                rng.random_range(area.start.y..area.end.y),
            ),
            hp: Hp::new(
                rng.random_range(INITIAL_HP_MIN..INITIAL_HP_MAX),
                INITIAL_HP_MAX,
            ),
            dir: dirs.choose(&mut rand::rng()).unwrap().to_owned(),
            velocity: rng
                .random_range(INITIAL_VELOCITY_MIN..INITIAL_VELOCITY_MAX),
            glyph: ' ',
            ticks_since_glyph_update: 0,
        }
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

    /// Updates direction to follow the player
    fn update_dir(&mut self, player: &Player) {
        self.dir = self.coord.dir_to(player.coord());
    }

    /// Detects contact with the player and subtracts
    fn update_hp(&mut self, game: &Game) {
        let mut total_dmg: Hp = game.bullets.iter().fold(
            Hp::new(0.0, f64::INFINITY),
            |acc, entity| {
                if self.coord.is_near(entity.coord()) {
                    acc + entity.hp()
                } else {
                    acc
                }
            },
        );
        if self.coord.is_near(game.player.coord()) {
            total_dmg += game.player.hp();
        }
        self.hp -= total_dmg;
    }

    fn step(&mut self) {
        self.coord.step(self.velocity, self.dir);
    }

    /// Shoots a bullet in current direction
    fn shoot(&self, game: &mut Game) {
        let mut coord = self.coord;
        coord.step(1.0, self.dir); // avoid self-kill
        game.bullets.push(Bullet::new(
            coord,
            Hp::new(ZOMBIE_BULLET_HP, ZOMBIE_BULLET_HP),
            self.dir,
        ));
    }

    /// Performs a random action with weighted choice
    fn act(&mut self, game: &mut Game) {
        let (action, _) = ACTIONS
            .choose_weighted(&mut rand::rng(), |(_, w)| *w)
            .unwrap();
        match action {
            Action::Shoot => self.shoot(game),
            Action::Move => self.step(),
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

    fn tick(&self, game: &mut Game) -> Option<Self> {
        let mut next = *self;
        // check hp before update, so reaching zero hp after update_hp returns
        // Some one last time
        if !next.hp.is_alive() {
            return None;
        }
        next.update_glyph();
        next.update_hp(game);
        next.update_dir(&game.player);
        next.act(game);
        Some(next)
    }
}
