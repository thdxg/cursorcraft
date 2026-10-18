mod player;

pub use player::*;

pub struct Game {
    pub player: Player,
    pub exit: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            exit: false,
        }
    }
}
