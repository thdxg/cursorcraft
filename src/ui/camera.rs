use crate::world::components::{Cell, Coordinate};
use ratatui::prelude::*;

/// Represents the visible area of the world
#[derive(Default)]
pub struct Camera {
    /// Cell of the center of visible area
    center: Cell,
    area: Rect,
    pub cursor_pos: Option<Position>,
    max_player_offset: Cell,
}

impl Camera {
    pub fn update_area(&mut self, area: Rect) {
        self.area = area;
        self.max_player_offset =
            Cell::new(i32::from(area.width) / 4, i32::from(area.height) / 4);
    }

    pub fn update_center(&mut self, player_coord: Coordinate) {
        let next = Cell::new(
            self.center.x.clamp(
                player_coord.cell().x - self.max_player_offset.x,
                player_coord.cell().x + self.max_player_offset.x,
            ),
            self.center.y.clamp(
                player_coord.cell().y - self.max_player_offset.y,
                player_coord.cell().y + self.max_player_offset.y,
            ),
        );
        self.center = next;
    }

    /// Returns current view's top-left cell in the world.
    pub fn start_cell(&self) -> Cell {
        let offset = Cell::new(
            i32::from(self.area.width) / 2,
            i32::from(self.area.height) / 2,
        );
        self.center - offset
    }

    /// Translates world coordinate to view position of an entity.
    /// Returns None when the entity is outside the visible area.
    pub fn get_pos(&self, coord: Coordinate) -> Option<Position> {
        let view_start_cell = self.start_cell();
        let rel_cell = coord.cell() - view_start_cell;
        match (u16::try_from(rel_cell.x), u16::try_from(rel_cell.y)) {
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
    pub fn update_cursor_pos(&mut self, coord: Coordinate) {
        self.cursor_pos = self.get_pos(coord);
    }
}
