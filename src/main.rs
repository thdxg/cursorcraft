mod app;
mod game;
mod ui;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App::new();
    app.run()?;
    Ok(())
}
