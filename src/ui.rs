mod stats;

use color_eyre::Result;
use crossterm::event;
use ratatui::{prelude::*, widgets::Block};
pub use stats::*;

use crate::game;

impl Widget for &game::Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]);
        let [header, main] = area.layout(&vertical);
        PlayerStats::from(&self.player).render(header, buf);
        Block::new().title("main").render(main, buf);
    }
}

impl game::Game {
    pub fn run(mut self) -> Result<()> {
        ratatui::run(|terminal| -> Result<()> {
            while !self.exit {
                terminal.draw(|frame| {
                    frame.render_widget(&self, frame.area());
                })?;
                self.handle_event()?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            event::Event::Key(key_event) => todo!(),
            event::Event::Resize(_, _) => todo!(),
            _ => {}
        }
        Ok(())
    }
}
