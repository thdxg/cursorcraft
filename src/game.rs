use crate::{
    ui::SceneState,
    world::{World, components::Direction},
};
use color_eyre::Result;
use core::fmt;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, KeyCode, KeyModifiers},
    execute,
};
use std::{fmt::Display, thread, time::Duration};

const FPS: u64 = 60;
const PLAYER_SPEED: f64 = 1.0;

#[derive(Default)]
pub struct Game {
    pub world: World,
    pub scene_state: SceneState,
    pub status: GameStatus,
}

impl Game {
    pub fn run(mut self) -> Result<()> {
        self.world.spawn_player();

        // set cursor style to block
        execute!(std::io::stdout(), SetCursorStyle::SteadyBlock)?;

        // game loop
        ratatui::run(|terminal| -> Result<()> {
            while self.status != GameStatus::Exited {
                if let GameStatus::Playing(_) = self.status {
                    self.world.tick();
                }
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
        })?;
        Ok(())
    }

    fn reset(&mut self) {
        self.world = World::default();
        self.scene_state = SceneState::default();
        self.status = GameStatus::Playing(GameMode::Normal);
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
                        .expect("if the player exists, velocity must exist");
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
                        .expect("if the player exists, velocity must exist");
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
                        .expect("if the player exists, velocity must exist");
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
                        .expect("if the player exists, velocity must exist");
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum GameStatus {
    Playing(GameMode),
    Paused,
    Finished,
    Exited,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Playing(GameMode::default())
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Insert,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameMode::Normal => f.write_str("NORMAL"),
            GameMode::Insert => f.write_str("INSERT"),
        }
    }
}
