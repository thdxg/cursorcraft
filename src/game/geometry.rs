#[derive(Default)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn step(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }
}
