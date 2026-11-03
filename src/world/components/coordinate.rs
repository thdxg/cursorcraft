use crate::world::components::{Direction, Velocity};
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn cell(&self) -> Cell {
        #[allow(clippy::cast_possible_truncation)]
        Cell {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    pub fn step(&self, vel: Velocity) -> Self {
        let mut next = *self;
        match vel.dir {
            Direction::Up => next.y -= vel.speed,
            Direction::Down => next.y += vel.speed,
            Direction::Left => next.x -= vel.speed,
            Direction::Right => next.x += vel.speed,
        }
        next
    }

    pub fn dir_to(self, other: Self) -> Direction {
        let rel = self - other;
        if rel.x.abs() > rel.y.abs() {
            if rel.x.is_sign_positive() {
                Direction::Left
            } else {
                Direction::Right
            }
        } else {
            if rel.y.is_sign_positive() {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Cell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Cell {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Cell {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Cell {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
