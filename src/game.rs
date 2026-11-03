mod controls;
mod events;
pub mod status;

use crate::{
    game::status::{GameMode, GameStatus},
    ui::camera::Camera,
    world::World,
};
use color_eyre::Result;
use crossterm::{cursor::SetCursorStyle, execute};
use std::{thread, time::Duration};

const FPS: u64 = 60;

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
            loop {
                self.update_cursor()?;
                terminal.draw(|frame| {
                    frame.render_widget(&mut self, frame.area());
                    if let Some(cursor_pos) = self.camera.cursor_pos {
                        frame.set_cursor_position(cursor_pos);
                    }
                })?;
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
                    GameStatus::Exited => {
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(1_000 / FPS));
            }
            Ok(())
        })?;
        Ok(())
    }

    fn update_cursor(&mut self) -> Result<()> {
        if let GameStatus::Playing(mode) = self.status {
            let cursor_style = if mode == GameMode::Normal {
                SetCursorStyle::SteadyBlock
            } else {
                SetCursorStyle::SteadyBar
            };
            execute!(std::io::stdout(), cursor_style)?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.world = World::new();
        self.camera = Camera::default();
        self.status = GameStatus::default();
    }
}
