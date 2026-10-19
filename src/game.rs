pub mod action;
pub mod entities;
pub mod geometry;

pub use action::*;
pub use entities::*;

#[derive(Default)]
pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Move(dir) => self.player.step(dir),
            _ => todo!(),
        }
    }
}
