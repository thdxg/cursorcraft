use ratatui::{prelude::*, widgets::Paragraph};

use crate::game::{Game, status::GameStatus};

pub struct Footer<'a>(&'a Game);

impl<'a> Footer<'a> {
    pub fn new(game: &'a Game) -> Self {
        Self(game)
    }
}

impl Widget for Footer<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let Self(game) = self;
        let [left, right] = area.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]));

        // kill count
        let killed = game.world.stats.zombies_killed;
        Paragraph::new(format!("killed: {killed}")).render(left, buf);

        // game mode
        let label = match game.status {
            GameStatus::Playing(mode) => format!("--{mode}--"),
            GameStatus::Paused => ":PAUSED".to_string(),
            GameStatus::Finished => ":FINISHED".to_string(),
            GameStatus::Exited => ":EXITED".to_string(),
        };
        Paragraph::new(label)
            .alignment(Alignment::Right)
            .render(right, buf);
    }
}
