use crate::game;
use ratatui::{
    prelude::*,
    widgets::{LineGauge, Paragraph},
};

pub struct Header<'a>(pub &'a game::Game);

impl<'a> Header<'a> {
    pub fn new(game: &'a game::Game) -> Self {
        Self(game)
    }
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let Self(game) = self;
        let [left, right] = area.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]));

        // player hp
        LineGauge::default()
            .ratio(game.player.hp.current / game.player.hp.max)
            .label("HP")
            .filled_symbol(" ")
            .unfilled_symbol(" ")
            .filled_style(Style::new().on_red())
            .unfilled_style(Style::new().on_black())
            .render(left, buf);

        // player position
        Paragraph::new(format!(
            "x: {}, y: {}",
            game.player.pos.x, game.player.pos.y
        ))
        .alignment(Alignment::Right)
        .render(right, buf);
    }
}
