use crate::{
    game::{
        Game,
        status::{GameMode, GameStatus},
    },
    world::{
        components::{Coordinate, Direction},
        entity::EntityKind,
    },
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

    #[allow(clippy::too_many_lines)]
    fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != event::KeyEventKind::Press {
            return;
        }
        match (self.status, key.code, key.modifiers) {
            (GameStatus::Playing(GameMode::Normal), key, modifier) => match key
            {
                KeyCode::Char('q') => {
                    if modifier == KeyModifiers::NONE {
                        self.status = GameStatus::Exited;
                    }
                }
                KeyCode::Char('h') => match modifier {
                    KeyModifiers::NONE => {
                        self.move_player(Direction::Left);
                    }
                    KeyModifiers::CONTROL => {
                        self.move_camera(Direction::Left);
                    }
                    _ => {}
                },
                KeyCode::Char('j') => match modifier {
                    KeyModifiers::NONE => {
                        self.move_player(Direction::Down);
                    }
                    KeyModifiers::CONTROL => {
                        self.move_camera(Direction::Down);
                    }
                    _ => {}
                },
                KeyCode::Char('k') => match modifier {
                    KeyModifiers::NONE => {
                        self.move_player(Direction::Up);
                    }
                    KeyModifiers::CONTROL => {
                        self.move_camera(Direction::Up);
                    }
                    _ => {}
                },
                KeyCode::Char('l') => match modifier {
                    KeyModifiers::NONE => {
                        self.move_player(Direction::Right);
                    }
                    KeyModifiers::CONTROL => {
                        self.move_camera(Direction::Right);
                    }
                    _ => {}
                },
                KeyCode::Char('i') => {
                    if modifier == KeyModifiers::NONE {
                        self.status = GameStatus::Playing(GameMode::Insert);
                    }
                }
                KeyCode::Char('a') => {
                    if modifier == KeyModifiers::NONE {
                        self.move_player(Direction::Right);
                        self.status = GameStatus::Playing(GameMode::Insert);
                        // aaaaa
                    }
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
                KeyCode::Char('o') => self.place_block(),
                KeyCode::Char(' ') => self.move_player(Direction::Right),
                KeyCode::Enter => {
                    // TODO: move this to game controls; implement find_next_word_start, find_prev_word_end, etc. for w, e, b movement
                    // find start of consecutive blocks
                    let player = self
                        .world
                        .player()
                        .expect("player must exist in Playing status");
                    let mut coord = self.world.coordinate[player.id]
                        .expect("coordinate must exist for a player");
                    let is_block = |c: Coordinate| {
                        self.world
                            .occupancy
                            .get(&c.cell())
                            .is_some_and(|e| e.kind == EntityKind::Block)
                    };
                    coord.y += 1.0;
                    while is_block(coord.left().up()) && !is_block(coord.left())
                    {
                        coord = coord.left();
                    }
                    self.move_player_to(coord);
                }
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
