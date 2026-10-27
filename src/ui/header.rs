use crate::{game::Game, world::components::Coordinate};
use ratatui::{
    prelude::*,
    widgets::{LineGauge, Paragraph},
};

pub struct Header<'a>(&'a Game);

impl<'a> Header<'a> {
    pub fn new(game: &'a Game) -> Self {
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

        let player = game.world.player();
        let health_ratio = match player {
            Some(player) => game.world.health[player.id]
                .expect("if the player exists, health must exist")
                .ratio(),
            None => 0.0,
        };
        let Coordinate { x, y } = match player {
            Some(player) => game.world.coordinate[player.id]
                .expect("if the player exists, coordinate must exist"),
            None => Coordinate::new(0.0, 0.0),
        };

        // player hp
        LineGauge::default()
            .ratio(health_ratio)
            .label("HP")
            .filled_symbol(" ")
            .unfilled_symbol(" ")
            .filled_style(Style::new().on_red())
            .unfilled_style(Style::new().on_black())
            .render(left, buf);

        // player coordinate
        Paragraph::new(format!("x: {x}, y: {y}"))
            .alignment(Alignment::Right)
            .render(right, buf);
    }
}
