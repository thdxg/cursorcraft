pub mod camera;
mod footer;
mod header;
mod scene;

use crate::{
    game::Game,
    ui::{footer::Footer, header::Header, scene::Scene},
};
use ratatui::prelude::*;

impl Widget for &mut Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header, scene, footer] = area.layout(&vertical);
        Header::new(self).render(header, buf);
        Scene::new(&self.world).render(scene, buf, &mut self.camera);
        Footer::new(self).render(footer, buf);
    }
}
