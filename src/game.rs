mod events;
pub mod status;

use crate::{game::status::GameStatus, ui::camera::Camera, world::World};
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
        // set cursor style to block
        execute!(std::io::stdout(), SetCursorStyle::SteadyBlock)?;

        // game loop
        ratatui::run(|terminal| -> Result<()> {
            loop {
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

    fn reset(&mut self) {
        self.world = World::new();
        self.camera = Camera::default();
        self.status = GameStatus::default();
    }
}
