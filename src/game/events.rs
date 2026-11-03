use crate::{
    game::{
        Game,
        status::{GameMode, GameStatus},
    },
    world::components::Direction,
};
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
use std::time::Duration;

impl Game {
    pub(super) fn handle_event(&mut self) -> Result<()> {
        let is_event = event::poll(Duration::from_secs(0))?;
        if !is_event {
            return Ok(());
        }

        if let event::Event::Key(key_event) = event::read()? {
            self.handle_key_event(key_event);
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != event::KeyEventKind::Press {
            return;
        }
        match (self.status, key.code, key.modifiers) {
            (
                GameStatus::Playing(GameMode::Normal),
                key,
                KeyModifiers::NONE,
            ) => match key {
                KeyCode::Char('q') => {
                    self.status = GameStatus::Exited;
                }
                KeyCode::Char('h') => {
                    self.world.move_player(Direction::Left);
                }
                KeyCode::Char('j') => {
                    self.world.move_player(Direction::Down);
                }
                KeyCode::Char('k') => {
                    self.world.move_player(Direction::Up);
                }
                KeyCode::Char('l') => {
                    self.world.move_player(Direction::Right);
                }
                KeyCode::Char('i') => {
                    self.status = GameStatus::Playing(GameMode::Insert);
                }
                KeyCode::Char(' ') => {
                    self.status = GameStatus::Paused;
                }
                _ => {}
            },
            (
                GameStatus::Playing(GameMode::Insert),
                key,
                KeyModifiers::NONE,
            ) => match key {
                KeyCode::Char('o') => self.world.place_block(),
                KeyCode::Char('x') => todo!(),
                KeyCode::Esc => {
                    self.status = GameStatus::Playing(GameMode::Normal);
                }
                _ => {}
            },
            (GameStatus::Paused, KeyCode::Char(key), KeyModifiers::NONE) => {
                match key {
                    'q' => {
                        self.status = GameStatus::Exited;
                    }
                    ' ' => {
                        self.status = GameStatus::Playing(GameMode::Normal);
                    }
                    _ => {}
                }
            }
            (GameStatus::Finished, KeyCode::Char(key), KeyModifiers::NONE) => {
                match key {
                    'q' => {
                        self.status = GameStatus::Exited;
                    }
                    'r' => {
                        self.reset();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
