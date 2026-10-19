mod game;
mod ui;

use crate::{
    game::{Action, Game, geometry::Dir},
    ui::camera::Camera,
};
use color_eyre::Result;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
};
use std::time::Duration;

const FPS: u64 = 60;

#[derive(Default)]
pub struct App {
    pub game: Game,
    pub camera: Camera,
    exit: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App::default();
    app.run()?;
    Ok(())
}

impl App {
    pub fn run(mut self) -> Result<()> {
        execute!(std::io::stdout(), SetCursorStyle::SteadyBlock)?;
        ratatui::run(|terminal| -> Result<()> {
            while !self.exit {
                terminal.draw(|frame| {
                    frame.render_widget(&mut self, frame.area());
                    if let Some(cursor_pos) = self.camera.cursor_pos {
                        frame.set_cursor_position(cursor_pos);
                    }
                })?;
                self.handle_event()?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        let is_event = event::poll(Duration::from_millis(1_000 / FPS))?;
        if !is_event {
            return Ok(());
        }

        if let event::Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != event::KeyEventKind::Press {
            return;
        }

        match (key.code, key.modifiers) {
            (KeyCode::Char('q') | KeyCode::Esc, KeyModifiers::NONE)
            | (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.exit = true,
            (KeyCode::Char('h'), _) => self.game.apply(Action::Move(Dir::Left)),
            (KeyCode::Char('j'), _) => self.game.apply(Action::Move(Dir::Down)),
            (KeyCode::Char('k'), _) => self.game.apply(Action::Move(Dir::Up)),
            (KeyCode::Char('l'), _) => {
                self.game.apply(Action::Move(Dir::Right));
            }

            _ => {}
        }
    }
}
