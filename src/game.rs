pub mod entity;
pub mod event;
pub mod geometry;
mod glyph;

pub use entity::*;
pub use event::*;
use geometry::Coord;
use std::{
    fmt::{self, Display},
    iter, time,
};

const INITIAL_PLAYER_HP: Hp = Hp::new(100.0, 100.0);
const INITIAL_PLAYER_VELOCITY: f64 = 1.0;
const INITIAL_PLAYER_COORD: Coord = Coord::new(0.0, 0.0);
const INITIAL_ZOMBIE_COUNT: u64 = 10;
const ZOMBIE_SPAWN_OFFSET_MAX: Coord = Coord::new(100.0, 100.0);
const ZOMBIE_SPAWN_CHANCE: f64 = 0.05;

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub zombies: Vec<Zombie>,
    pub bullets: Vec<Bullet>,
    pub blocks: Vec<Block>,
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
            blocks: vec![],
            status: Status::Playing(Mode::Normal),
            started_at: time::Instant::now(),
        };
        (0..INITIAL_ZOMBIE_COUNT).for_each(|_| game.spawn_zombie());

        game
    }

    pub fn tick(&self) -> Self {
        let mut next = self.clone();
        if let Status::Playing(_) = self.status {
        } else {
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

    pub fn apply(&mut self, action: Event) {
        match action {
            Event::MovePlayer(dir) => {
                self.player.step(dir);
            }
            Event::PlaceBlock(block) => {
                self.blocks.push(block);
                self.player.step(geometry::Dir::Right);
            }
            Event::Damage { target, amount } => todo!(),
        }
    }

    pub fn entities(&self) -> impl Iterator<Item = &dyn Entity> {
        iter::once(&self.player as &dyn Entity)
            .chain(self.zombies.iter().map(|e| e as &dyn Entity))
            .chain(self.bullets.iter().map(|e| e as &dyn Entity))
            .chain(self.blocks.iter().map(|e| e as &dyn Entity))
    }

    fn spawn_zombie(&mut self) {
        let player_coord = self.player.coord();
        let area = (player_coord - ZOMBIE_SPAWN_OFFSET_MAX)
            ..(player_coord + ZOMBIE_SPAWN_OFFSET_MAX);
        let zombie = Zombie::spawn_in(area);
        self.zombies.push(zombie);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Playing(Mode),
    Paused,
    Finished,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Playing(_) => f.write_str("Playing"),
            Status::Paused => f.write_str("Paused"),
            Status::Finished => f.write_str("Game Over"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal => f.write_str("NORMAL"),
            Mode::Insert => f.write_str("INSERT"),
        }
    }
}
