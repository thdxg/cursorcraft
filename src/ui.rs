mod ending;
mod entity;
mod header;
pub mod scene;

use crate::App;
use ending::Ending;
use header::Header;
use ratatui::prelude::*;
use scene::Scene;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [header, main] = area.layout(&vertical);
        Header::new(&self.game).render(header, buf);
        if self.game.finished {
            Ending::new().render(area, buf);
        } else {
            Scene::new(&self.game).render(main, buf, &mut self.view);
        }
    }
}
