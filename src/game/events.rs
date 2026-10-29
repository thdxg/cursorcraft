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

const PLAYER_SPEED: f64 = 1.0;

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
                KeyCode::Char(key),
                KeyModifiers::NONE,
            ) => match key {
                'q' => {
                    self.status = GameStatus::Exited;
                }
                'h' => {
                    let player = self
                        .world
                        .player()
                        .expect("player must exist in Playing status");
                    let vel = self.world.velocity[player.id]
                        .as_mut()
                        .expect("velocity must exist for a player");
                    vel.dir = Direction::Left;
                    vel.speed = PLAYER_SPEED;
                }
                'j' => {
                    let player = self
                        .world
                        .player()
                        .expect("player must exist in Playing status");
                    let vel = self.world.velocity[player.id]
                        .as_mut()
                        .expect("velocity must exist for a player");
                    vel.dir = Direction::Down;
                    vel.speed = PLAYER_SPEED;
                }
                'k' => {
                    let player = self
                        .world
                        .player()
                        .expect("player must exist in Playing status");
                    let vel = self.world.velocity[player.id]
                        .as_mut()
                        .expect("velocity must exist for a player");
                    vel.dir = Direction::Up;
                    vel.speed = PLAYER_SPEED;
                }
                'l' => {
                    let player = self
                        .world
                        .player()
                        .expect("player must exist in Playing status");
                    let vel = self.world.velocity[player.id]
                        .as_mut()
                        .expect("velocity must exist for a player");
                    vel.dir = Direction::Right;
                    vel.speed = PLAYER_SPEED;
                }
                ' ' => {
                    self.status = GameStatus::Paused;
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
