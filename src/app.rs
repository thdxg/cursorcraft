use crate::{
    game::{self, Action, Game, geometry},
    ui::{self},
};
use color_eyre::Result;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
};
use std::{thread, time::Duration};

const FPS: u64 = 60;

pub struct App {
    pub game: Game,
    pub scene_state: ui::scene::SceneState,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            scene_state: ui::scene::SceneState::default(),
            exit: false,
        }
    }

    pub fn run(self) -> Result<()> {
        self.draw()?;

        Ok(())
    }

    fn draw(mut self) -> Result<()> {
        // set cursor to block mode
        execute!(std::io::stdout(), SetCursorStyle::SteadyBlock)?;
        ratatui::run(|terminal| -> Result<()> {
            while !self.exit {
                self.game = self.game.tick();
                self.handle_event()?;
                terminal.draw(|frame| {
                    frame.render_widget(&mut self, frame.area());
                    if let Some(cursor_pos) = self.scene_state.cursor_pos {
                        frame.set_cursor_position(cursor_pos);
                    }
                })?;
                thread::sleep(Duration::from_millis(1_000 / FPS));
            }
            Ok(())
        })
    }

    fn handle_event(&mut self) -> Result<()> {
        let is_event = event::poll(Duration::from_secs(0))?;
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

        match (self.game.status, key.code, key.modifiers) {
            (_, KeyCode::Char('q') | KeyCode::Esc, KeyModifiers::NONE)
            | (_, KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.exit = true;
            }
            (game::Status::Playing, KeyCode::Char('h'), KeyModifiers::NONE) => {
                self.game.apply(Action::Move(geometry::Dir::Left));
            }
            (game::Status::Playing, KeyCode::Char('j'), KeyModifiers::NONE) => {
                self.game.apply(Action::Move(geometry::Dir::Down));
            }
            (game::Status::Playing, KeyCode::Char('k'), KeyModifiers::NONE) => {
                self.game.apply(Action::Move(geometry::Dir::Up));
            }
            (game::Status::Playing, KeyCode::Char('l'), KeyModifiers::NONE) => {
                self.game.apply(Action::Move(geometry::Dir::Right));
            }
            (game::Status::Playing, KeyCode::Char(' '), KeyModifiers::NONE) => {
                self.game.status = game::Status::Paused;
            }
            (game::Status::Paused, KeyCode::Char(' '), KeyModifiers::NONE) => {
                self.game.status = game::Status::Playing;
            }
            (game::Status::Finished, KeyCode::Enter, KeyModifiers::NONE) => {
                self.game = game::Game::new();
            }
            _ => {}
        }
    }
}
