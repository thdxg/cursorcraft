pub mod camera;
mod footer;
mod header;
mod scene;
mod summary;

use crate::{
    game::Game,
    ui::{footer::Footer, header::Header, scene::Scene, summary::Summary},
};
use ratatui::prelude::*;

impl Widget for &mut Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header, main, footer] = area.layout(&vertical);
        Header::new(self).render(header, buf);
        Scene::new(&self.world).render(main, buf, &mut self.camera);
        Summary::new(self).render(main, buf);
        Footer::new(self).render(footer, buf);
    }
}
