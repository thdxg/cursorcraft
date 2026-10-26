use crate::{
    game::{self, Event, Entity, Game, Mode, Status, geometry},
    ui::{self, scene},
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

    fn reset(&mut self) {
        self.game = game::Game::new();
        self.scene_state = scene::SceneState::default();
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
            // normal mode
            (
                Status::Playing(Mode::Normal)
                | Status::Paused
                | Status::Finished,
                KeyCode::Char('q'),
                KeyModifiers::NONE,
            ) => {
                self.exit = true;
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char('h'),
                KeyModifiers::NONE,
            ) => {
                self.game.apply(Event::MovePlayer(geometry::Dir::Left));
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char('j'),
                KeyModifiers::NONE,
            ) => {
                self.game.apply(Event::MovePlayer(geometry::Dir::Down));
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char('k'),
                KeyModifiers::NONE,
            ) => {
                self.game.apply(Event::MovePlayer(geometry::Dir::Up));
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char('l'),
                KeyModifiers::NONE,
            ) => {
                self.game.apply(Event::MovePlayer(geometry::Dir::Right));
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char(' '),
                KeyModifiers::NONE,
            ) => {
                self.game.status = game::Status::Paused;
            }
            (
                Status::Playing(Mode::Normal),
                KeyCode::Char('i'),
                KeyModifiers::NONE,
            ) => {
                self.game.status = game::Status::Playing(Mode::Insert);
            }
            // insert mode
            (
                Status::Playing(Mode::Insert),
                KeyCode::Char(c),
                KeyModifiers::NONE,
            ) => {
                if let Ok(kind) = game::BlockKind::try_from(c) {
                    let block =
                        game::Block::new(kind, self.game.player.coord());
                    self.game.apply(Event::PlaceBlock(block));
                }
            }
            (
                Status::Playing(Mode::Insert),
                KeyCode::Esc,
                KeyModifiers::NONE,
            )
            | (game::Status::Paused, KeyCode::Char(' '), KeyModifiers::NONE) => {
                self.game.status = game::Status::Playing(Mode::Normal);
            }

            // etc.
            (game::Status::Finished, KeyCode::Enter, KeyModifiers::NONE) => {
                self.reset();
            }
            _ => {}
        }
    }
}
