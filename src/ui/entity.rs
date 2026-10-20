use super::scene;
use crate::game;
use ratatui::prelude::*;

impl StatefulWidget for &game::Zombie {
    type State = scene::SceneState;

    fn render(self, _: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Some(pos) = state.get_pos(self) else {
            return;
        };
        let Some(cell) = buf.cell_mut(pos) else {
            return;
        };
        cell.set_char(self.glyph);
    }
}
