use crate::game::{self};
use ratatui::{prelude::*, widgets::Paragraph};

pub struct Footer<'a>(pub &'a game::Game);

impl<'a> Footer<'a> {
    pub fn new(game: &'a game::Game) -> Self {
        Self(game)
    }
}

impl Widget for Footer<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let Self(game) = self;
        let [_, right] = area.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]));

        // game mode
        if let game::Status::Playing(mode) = game.status {
            Paragraph::new(format!("--{mode}--"))
                .alignment(Alignment::Right)
                .render(right, buf);
        }
    }
}
