const HIT_BOX: f64 = 1.0;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn step(&mut self, amount: f64, dir: Dir) {
        match dir {
            Dir::Up => self.y -= amount,
            Dir::Down => self.y += amount,
            Dir::Left => self.x -= amount,
            Dir::Right => self.x += amount,
        }
    }

    pub fn relative_to(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn dir_to(&self, other: &Self) -> Dir {
        let hor = self.x - other.x;
        let ver = self.y - other.y;
        if hor.abs() > ver.abs() {
            if hor.is_sign_positive() {
                Dir::Left
            } else {
                Dir::Right
            }
        } else {
            if ver.is_sign_positive() {
                Dir::Up
            } else {
                Dir::Down
            }
        }
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let hor = (self.x - other.x).abs();
        let ver = (self.y - other.y).abs();
        (hor.powi(2) + ver.powi(2)).sqrt()
    }

    pub fn is_near(&self, other: &Self) -> bool {
        self.distance_to(other) <= HIT_BOX
    }
}
