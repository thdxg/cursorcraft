mod entity;
mod footer;
mod header;
pub mod scene;
mod summary;

use crate::{App, game};
use footer::Footer;
use header::Header;
use ratatui::prelude::*;
use scene::Scene;
use summary::Summary;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header, main, footer] = area.layout(&vertical);
        Header::new(&self.game).render(header, buf);
        match self.game.status {
            game::Status::Playing(_) => {
                Scene::new(&self.game).render(main, buf, &mut self.scene_state);
            }
            game::Status::Paused | game::Status::Finished => {
                Summary::new(&self.game).render(main, buf);
            }
        }
        Footer::new(&self.game).render(footer, buf);
    }
}
