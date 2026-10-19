use ratatui::layout::Position;

#[derive(Default)]
pub struct Camera {
    pub center_x: i64,
    pub center_y: i64,
    pub width: u16,
    pub height: u16,
    pub cursor_pos: Option<Position>,
}
