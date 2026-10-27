use crate::world::components::{Direction, Velocity};
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

const HIT_BOX: f64 = 1.0;

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn step(&mut self, vel: Velocity) {
        match vel.dir {
            Direction::Up => self.y -= vel.speed,
            Direction::Down => self.y += vel.speed,
            Direction::Left => self.x -= vel.speed,
            Direction::Right => self.x += vel.speed,
        }
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

    pub fn is_near(self, other: Self) -> bool {
        let rel = self - other;
        rel.x.powi(2) + rel.y.powi(2) <= HIT_BOX.powi(2)
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
