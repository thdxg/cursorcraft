use crate::game::{Game, status::GameStatus};
use ratatui::{
    prelude::*,
    widgets::{Block, Cell, Clear, Padding, Row, Table},
};

pub struct Summary<'a>(&'a Game);

impl<'a> Summary<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self(game)
    }
}

impl Widget for Summary<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let Self(game) = self;
        if let GameStatus::Playing(_) = game.status {
            return;
        }
        let stats = &game.world.stats;

        let area = area
            .centered_horizontally(Constraint::Length(30))
            .centered_vertically(Constraint::Length(5));

        Clear.render(area, buf);

        let rows = [("zombies killed", stats.zombies_killed.to_string())].map(
            |(label, value)| {
                Row::new([
                    Cell::from(Text::from(label)),
                    Cell::from(
                        Text::from(value).alignment(HorizontalAlignment::Right),
                    ),
                ])
            },
        );
        let widths = [Constraint::Fill(1), Constraint::Length(5)];
        let table = Table::new(rows, widths);
        let table = match game.status {
            GameStatus::Paused => table.block(
                Block::new()
                    .title(Line::from("paused").centered().bold())
                    .bg(Color::Black)
                    .padding(Padding::top(1)),
            ),
            GameStatus::Finished => table.block(
                Block::new()
                    .title(Line::from("game over").centered().bold())
                    .bg(Color::Black)
                    .padding(Padding::uniform(1)),
            ),
            _ => return,
        };

        Widget::render(table, area, buf);
    }
}
