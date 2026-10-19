pub mod camera;
pub mod header;
pub mod scene;

use crate::App;
pub use header::*;
use ratatui::prelude::*;
pub use scene::*;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [header, scene] = area.layout(&vertical);
        Header::new(&self.game).render(header, buf);
        Scene::new(&self.game).render(scene, buf, &mut self.camera);
    }
}
