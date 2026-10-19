use crate::{game, ui::camera::Camera};
use ratatui::{prelude::*, widgets::Block};

pub struct Scene<'a> {
    pub game: &'a game::Game,
}

impl<'a> Scene<'a> {
    pub fn new(game: &'a game::Game) -> Self {
        Self { game }
    }
}

impl StatefulWidget for Scene<'_> {
    type State = Camera;
    fn render(self, area: Rect, buf: &mut Buffer, camera: &mut Self::State) {
        let game = self.game;
        let block = Block::bordered();
        let inner = block.inner(area);
        block.render(area, buf);

        // calculate cursor position
        camera.width = inner.width;
        camera.height = inner.height;
        let start_x = camera.center_x - i64::from(camera.width) / 2;
        let start_y = camera.center_y - i64::from(camera.height) / 2;
        camera.cursor_pos = match (
            u16::try_from(game.player.pos.x - start_x),
            u16::try_from(game.player.pos.y - start_y),
        ) {
            (Ok(x), Ok(y)) => {
                let pos = Position::new(
                    inner.x.saturating_add(x),
                    inner.y.saturating_add(y),
                );
                inner.contains(pos).then_some(pos)
            }
            _ => None,
        }
    }
}
