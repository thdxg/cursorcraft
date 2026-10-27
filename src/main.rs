mod game;
mod ui;
mod world;

use crate::game::Game;
use color_eyre::Result;

fn main() -> Result<()> {
    let game = Game::new();
    game.run()
}
