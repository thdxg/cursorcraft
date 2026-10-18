use color_eyre::Result;

mod game;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let game = game::Game::new();
    game.run()?;
    Ok(())
}
