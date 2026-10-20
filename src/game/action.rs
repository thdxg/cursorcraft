use super::geometry::Dir;

#[derive(Copy, Clone)]
pub enum Action {
    Move(Dir),
    Load,
    Shoot(Dir),
}
