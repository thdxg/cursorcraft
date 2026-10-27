use crate::{
    ui::camera::Camera,
    world::{World, entity::EntityKind},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub struct Scene<'a>(&'a World);

impl<'a> Scene<'a> {
    pub fn new(ui: &'a World) -> Self {
        Self(ui)
    }
}

impl StatefulWidget for Scene<'_> {
    type State = Camera;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Self(world) = self;
        let block =
            Block::bordered().border_style(Style::new().fg(Color::Black));
        let inner = block.inner(area);
        block.render(area, buf);

        if let Some(player) = world.player() {
            let coord = world.coordinate[player.id]
                .expect("if the player exists, coordinate must exist");
            state.update_cursor_pos(coord);
            state.update_center(coord);
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

        let info_panel = Block::new();
        Paragraph::new(format!(
            "zombies: {}",
            world
                .entities
                .iter()
                .filter(|e| e.kind == EntityKind::Zombie)
                .count()
        ))
        .block(info_panel)
        .render(Rect::new(inner.x, inner.y, 15, 5), buf);
    }
}
