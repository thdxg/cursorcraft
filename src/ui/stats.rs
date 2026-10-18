use ratatui::{
    prelude::*,
    widgets::{Block, Borders, LineGauge, Paragraph},
};

use crate::game;

pub struct PlayerStats<'a>(pub &'a game::Player);

impl<'a> From<&'a game::Player> for PlayerStats<'a> {
    fn from(p: &'a game::Player) -> Self {
        Self(p)
    }
}

impl Widget for PlayerStats<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let p = self.0;
        let block = Block::new().borders(Borders::BOTTOM);
        let [left, right] = block.inner(area).layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]));
        HpBar::from(p).render(left, buf);
        PosLabel::from(p).render(right, buf);
    }
}

struct HpBar {
    ratio: f64,
}

impl From<&game::Player> for HpBar {
    fn from(p: &game::Player) -> Self {
        Self {
            ratio: p.hp.current / p.hp.max,
        }
    }
}

impl Widget for HpBar {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        LineGauge::default()
            .ratio(self.ratio)
            .label("HP")
            .filled_symbol(" ")
            .unfilled_symbol(" ")
            .filled_style(Style::new().on_red())
            .unfilled_style(Style::new().on_black())
            .render(area, buf);
    }
}

struct PosLabel {
    x: i64,
    y: i64,
}

impl From<&game::Player> for PosLabel {
    fn from(p: &game::Player) -> Self {
        let (x, y) = p.pos;
        Self { x, y }
    }
}

impl Widget for PosLabel {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(format!("x: {}, y: {}", self.x, self.y))
            .alignment(Alignment::Right)
            .render(area, buf);
    }
}
