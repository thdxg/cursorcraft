use super::geometry::Dir;

pub enum Action {
    Move(Dir),
    Load,
    Shoot(Dir),
}
