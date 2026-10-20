pub mod action;
pub mod entity;
pub mod geometry;
mod glyph;

pub use action::*;
pub use entity::*;
use geometry::Coord;
use std::{
    fmt::{self, Display},
    time,
};

const INITIAL_PLAYER_HP: Hp = Hp::new(100.0, 100.0);
const INITIAL_PLAYER_VELOCITY: f64 = 1.0;
const INITIAL_PLAYER_COORD: Coord = Coord::new(0.0, 0.0);
const INITIAL_ZOMBIE_COUNT: u64 = 10;
const ZOMBIE_SPAWN_OFFSET_MAX: Coord = Coord::new(100.0, 100.0);
const ZOMBIE_SPAWN_CHANCE: f64 = 0.05;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Playing,
    Paused,
    Finished,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Playing => f.write_str("Playing"),
            Status::Paused => f.write_str("Paused"),
            Status::Finished => f.write_str("Game Over"),
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub zombies: Vec<Zombie>,
    pub bullets: Vec<Bullet>,
    pub status: Status,
    pub started_at: time::Instant,
}

impl Game {
    /// Creates a `Game` instance with some default settings.
    pub fn new() -> Self {
        let player = Player::new(
            INITIAL_PLAYER_COORD,
            INITIAL_PLAYER_HP,
            INITIAL_PLAYER_VELOCITY,
        );
        let mut game = Self {
            player,
            zombies: vec![],
            bullets: vec![],
            status: Status::Playing,
            started_at: time::Instant::now(),
        };
        (0..INITIAL_ZOMBIE_COUNT).for_each(|_| game.spawn_zombie());

        game
    }

    pub fn tick(&self) -> Self {
        let mut next = self.clone();
        if self.status != Status::Playing {
            return next;
        }
        if let Some(player) = self.player.tick(&mut next) {
            next.player = player;
        } else {
            next.status = Status::Finished;
        }
        next.bullets = self
            .bullets
            .iter()
            .filter_map(|e| e.tick(&mut next))
            .collect();
        next.zombies = self
            .zombies
            .iter()
            .filter_map(|e| e.tick(&mut next))
            .collect();
        if rand::random_bool(ZOMBIE_SPAWN_CHANCE) {
            next.spawn_zombie();
        }
        next
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Move(dir) => self.player.step(dir),
            _ => todo!(),
        }
    }

    fn spawn_zombie(&mut self) {
        let player_coord = self.player.coord();
        let area = (player_coord - ZOMBIE_SPAWN_OFFSET_MAX)
            ..(player_coord + ZOMBIE_SPAWN_OFFSET_MAX);
        let zombie = Zombie::spawn_in(area);
        self.zombies.push(zombie);
    }
}
