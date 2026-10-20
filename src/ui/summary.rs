use crate::game;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub struct Summary<'a>(pub &'a game::Game);

impl<'a> Summary<'a> {
    pub fn new(game: &'a game::Game) -> Self {
        Self(game)
    }
}

impl Widget for Summary<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let game = self.0;

        let block = Block::bordered();
        let inner = block.inner(area);
        let lines = vec![Line::from(format!("{}", game.status))];
        #[expect(
            clippy::cast_possible_truncation,
            reason = "lines length is under u16"
        )]
        let height = lines.len() as u16;
        block.render(area, buf);
        Paragraph::new(lines)
            .centered()
            .render(inner.centered_vertically(Constraint::Length(height)), buf);
    }
}
