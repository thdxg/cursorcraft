pub mod action;
pub mod entity;
pub mod geometry;
mod glyph;

pub use action::*;
pub use entity::*;
use geometry::{Dir, Pos};
use rand::{RngExt, seq::IndexedRandom};

const INITIAL_PLAYER_HP: f64 = 100.0;
const INITIAL_PLAYER_VELOCITY: f64 = 1.0;
const INITIAL_ZOMBIE_OFFSET_MAX: f64 = 40.0;
const INITIAL_ZOMBIE_COUNT: u16 = 10;
const INITIAL_ZOMBIE_HP_MIN: f64 = 20.0;
const INITIAL_ZOMBIE_HP_MAX: f64 = 100.0;
const INITIAL_ZOMBIE_VELOCITY_MIN: f64 = 0.01;
const INITIAL_ZOMBIE_VELOCITY_MAX: f64 = 0.03;

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub zombies: Vec<Zombie>,
    pub bullets: Vec<Bullet>,
    pub finished: bool,
}

impl Game {
    pub fn new() -> Self {
        let player = Player::new(
            Pos::default(),
            Hp::new(INITIAL_PLAYER_HP, INITIAL_PLAYER_HP),
            INITIAL_PLAYER_VELOCITY,
        );
        let mut game = Self {
            player,
            zombies: vec![],
            bullets: vec![],
            finished: false,
        };
        for _ in 0..INITIAL_ZOMBIE_COUNT {
            game.spawn_zombie();
        }
        game.tick();

        game
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        let game = &self.clone();
        match self.player.tick(game) {
            Some(player) => {
                self.player = player;
            }
            None => {
                self.finished = true;
            }
        }
        self.zombies = self
            .zombies
            .iter_mut()
            .filter_map(|e| e.tick(game))
            .collect();
        self.bullets = self
            .bullets
            .iter_mut()
            .filter_map(|e| e.tick(game))
            .collect();
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Move(dir) => self.player.update_pos(dir),
            _ => todo!(),
        }
    }

    fn spawn_zombie(&mut self) {
        let mut rng = rand::rng();
        let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        let zombie = Zombie::new(
            Pos::new(
                rng.random_range(
                    -INITIAL_ZOMBIE_OFFSET_MAX..INITIAL_ZOMBIE_OFFSET_MAX,
                ),
                rng.random_range(
                    -INITIAL_ZOMBIE_OFFSET_MAX..INITIAL_ZOMBIE_OFFSET_MAX,
                ),
            ),
            Hp::new(
                rng.random_range(INITIAL_ZOMBIE_HP_MIN..INITIAL_ZOMBIE_HP_MAX),
                INITIAL_ZOMBIE_HP_MAX,
            ),
            dirs.choose(&mut rand::rng()).unwrap().to_owned(),
            rng.random_range(
                INITIAL_ZOMBIE_VELOCITY_MIN..INITIAL_ZOMBIE_VELOCITY_MAX,
            ),
        );
        self.zombies.push(zombie);
    }
}
