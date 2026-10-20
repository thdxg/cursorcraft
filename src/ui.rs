mod summary;
mod entity;
mod header;
pub mod scene;

use crate::{App, game};
use summary::Summary;
use header::Header;
use ratatui::prelude::*;
use scene::Scene;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [header, main] = area.layout(&vertical);
        Header::new(&self.game).render(header, buf);
        match self.game.status {
            game::Status::Playing => {
                Scene::new(&self.game).render(main, buf, &mut self.scene_state);
            }
            game::Status::Paused | game::Status::Finished => {
                Summary::new(&self.game).render(main, buf);
            }
        }
    }
}
