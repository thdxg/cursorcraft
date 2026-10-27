use crate::{ui::SceneState, world::World};
use ratatui::{prelude::*, widgets::Block};

pub struct Scene<'a>(&'a World);

impl<'a> Scene<'a> {
    pub fn new(ui: &'a World) -> Self {
        Self(ui)
    }
}

impl StatefulWidget for Scene<'_> {
    type State = SceneState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Self(world) = self;
        let block =
            Block::bordered().border_style(Style::new().fg(Color::Black));
        let inner = block.inner(area);
        block.render(area, buf);

        match world.player() {
            Some(player) => {
                let coord = world.coordinate[player.id]
                    .expect("if the player exists, coordinate must exist");
                state.update_cursor_pos(coord);
                state.update_center(coord);
            }
            None => todo!(),
        }
        state.update_area(inner);
        world.entities.iter().for_each(|e| {
            let Some(coord) = world.coordinate[e.id] else {
                return;
            };
            let Some(glyph) = world.glyph[e.id] else {
                return;
            };
            let Some(pos) = state.get_pos(coord) else {
                return;
            };
            let Some(cell) = buf.cell_mut(pos) else {
                return;
            };
            cell.set_char(glyph);
        });
    }
}
