use std::ops::{Add, AddAssign, Sub, SubAssign};

const HIT_BOX: f64 = 1.0;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub const fn new(x: f64, y: f64) -> Self {
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

    pub fn dir_to(self, other: Self) -> Dir {
        let rel = self - other;
        if rel.x.abs() > rel.y.abs() {
            if rel.x.is_sign_positive() {
                Dir::Left
            } else {
                Dir::Right
            }
        } else {
            if rel.y.is_sign_positive() {
                Dir::Up
            } else {
                Dir::Down
            }
        }
    }

    pub fn distance_to(self, other: Self) -> f64 {
        let rel = self - other;
        (rel.x.powi(2) + rel.y.powi(2)).sqrt()
    }

    pub fn is_near(self, other: Self) -> bool {
        self.distance_to(other) <= HIT_BOX
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
