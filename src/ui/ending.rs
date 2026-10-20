use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub struct Ending {}

impl Ending {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Ending {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered();
        let inner = block.inner(area);
        Paragraph::new("Game Over")
            .block(block)
            .centered()
            .render(inner, buf);
    }
}
