mod controls;
mod events;
pub mod status;

use crate::{
    game::status::{GameMode, GameStatus},
    ui::camera::Camera,
    world::World,
};
use color_eyre::Result;
use crossterm::{cursor, execute};
use ratatui::Frame;
use std::{
    io::stdout,
    thread,
    time::{Duration, Instant},
};

const FPS: u64 = 60;
const FRAME_DUR: Duration = Duration::from_millis(1_000 / FPS);

pub struct Game {
    pub world: World,
    pub camera: Camera,
    pub status: GameStatus,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            camera: Camera::default(),
            status: GameStatus::default(),
        }
    }

    pub fn run(mut self) -> Result<()> {
        // game loop
        ratatui::run(|terminal| -> Result<()> {
            while self.status != GameStatus::Exited {
                let before = Instant::now();
                self.tick()?;
                terminal.draw(|frame| {
                    frame.render_widget(&mut self, frame.area());
                    self.update_cursor(frame);
                })?;
                let after = Instant::now();

                // sleep to synchronize to 60 fps
                thread::sleep(
                    FRAME_DUR
                        .checked_sub(after - before)
                        .unwrap_or(Duration::ZERO),
                );
            }
            Ok(())
        })?;
        Ok(())
    }

    fn tick(&mut self) -> Result<()> {
        match self.status {
            GameStatus::Playing(mode) => {
                self.world.tick();
                if self.world.player().is_none() {
                    self.status = GameStatus::Finished;
                } else {
                    self.status = GameStatus::Playing(mode);
                }
                self.handle_event()?;
            }
            GameStatus::Paused | GameStatus::Finished => {
                self.handle_event()?;
            }
            GameStatus::Exited => {}
        }
        Ok(())
    }

    /// Update cursor position and style based on game state.
    ///
    /// # Panics
    ///
    /// This function will panic when crossterm fails to set cursor style.
    fn update_cursor(&mut self, frame: &mut Frame) {
        match self.status {
            GameStatus::Playing(GameMode::Normal) => {
                execute!(stdout(), cursor::SetCursorStyle::SteadyBlock)
                    .unwrap();
                if let Some(cursor_pos) = self.camera.cursor_pos {
                    frame.set_cursor_position(cursor_pos);
                }
            }
            GameStatus::Playing(GameMode::Insert) => {
                execute!(stdout(), cursor::SetCursorStyle::SteadyBar).unwrap();
                if let Some(cursor_pos) = self.camera.cursor_pos {
                    frame.set_cursor_position(cursor_pos);
                }
            }
            _ => {
                execute!(std::io::stdout(), cursor::Hide).unwrap();
            }
        }
    }

    fn reset(&mut self) {
        self.world = World::new();
        self.camera = Camera::default();
        self.status = GameStatus::default();
    }
}
