use super::scene;
use crate::game;
use ratatui::prelude::*;

impl StatefulWidget for &dyn game::Entity {
    type State = scene::SceneState;

    fn render(self, _: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Some(pos) = state.get_pos(self) else {
            return;
        };
        let Some(cell) = buf.cell_mut(pos) else {
            return;
        };
        if let Some(e) = self.as_any().downcast_ref::<game::Zombie>() {
            cell.set_char(e.glyph).set_fg(Color::Red);
        } else if self.as_any().downcast_ref::<game::Bullet>().is_some() {
            cell.set_char('•');
        } else if let Some(e) = self.as_any().downcast_ref::<game::Block>() {
            cell.set_char(e.kind.into()).set_fg(Color::Red);
        }
    }
}
