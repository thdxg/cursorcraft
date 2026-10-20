use crate::game::{self, Entity, geometry};
use ratatui::{prelude::*, widgets::Block};

pub struct Scene<'a>(pub &'a game::Game);

impl<'a> Scene<'a> {
    pub fn new(game: &'a game::Game) -> Self {
        Self(game)
    }
}

impl StatefulWidget for Scene<'_> {
    type State = SceneState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let game = self.0;
        let block =
            Block::bordered().border_style(Style::new().fg(Color::Black));
        let inner = block.inner(area);
        block.render(area, buf);
        state.update_area(inner);
        state.update_cursor_pos(&game.player);
        state.update_center(game.player.coord());

        for e in &game.zombies {
            e.render(inner, buf, state);
        }
    }
}

/// Represents the visible area of the world
#[derive(Default)]
pub struct SceneState {
    /// Coordinate of the center of visible area
    center: geometry::Coord,
    area: Rect,
    pub cursor_pos: Option<Position>,
    max_player_offset: geometry::Coord,
}

impl SceneState {
    fn update_area(&mut self, area: Rect) {
        self.area = area;
        self.max_player_offset = geometry::Coord::new(
            f64::from(area.width) / 4.0,
            f64::from(area.height) / 4.0,
        );
    }

    fn update_center(&mut self, player_coord: geometry::Coord) {
        self.center.x = self.center.x.clamp(
            player_coord.x - self.max_player_offset.x,
            player_coord.x + self.max_player_offset.x,
        );
        self.center.y = self.center.y.clamp(
            player_coord.y - self.max_player_offset.y,
            player_coord.y + self.max_player_offset.y,
        );
    }

    /// Returns current view's top-left coordinate in the world.
    fn start_coord(&self) -> geometry::Coord {
        geometry::Coord::new(
            self.center.x - f64::from(self.area.width) / 2.0,
            self.center.y - f64::from(self.area.height) / 2.0,
        )
    }

    /// Translates world position to view position of an entity.
    /// Returns None when the entity is outside the visible area.
    pub fn get_pos(&self, entity: &dyn game::Entity) -> Option<Position> {
        let coord = entity.coord();
        let view_start_coord = self.start_coord();
        let rel_coord = coord - view_start_coord;
        #[expect(
            clippy::cast_possible_truncation,
            reason = "intentional truncation"
        )]
        match (
            u16::try_from(rel_coord.x.round() as i64),
            u16::try_from(rel_coord.y.round() as i64),
        ) {
            (Ok(rel_x), Ok(rel_y)) => {
                let abs_pos = Position::new(
                    self.area.x.saturating_add(rel_x),
                    self.area.y.saturating_add(rel_y),
                );
                self.area.contains(abs_pos).then_some(abs_pos)
            }
            _ => None,
        }
    }

    /// Updates `cursor_pos`, which is an absolute position on terminal screen.
    fn update_cursor_pos(&mut self, player: &game::Player) {
        self.cursor_pos = self.get_pos(player);
    }
}
