#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Velocity {
    pub speed: f64,
    pub dir: Direction,
}

impl Velocity {
    pub fn new(speed: f64, dir: Direction) -> Self {
        Self { speed, dir }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
