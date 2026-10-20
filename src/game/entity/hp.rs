use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Hp {
    current: f64,
    max: f64,
}

impl Hp {
    pub const fn new(current: f64, max: f64) -> Self {
        Self { current, max }
    }

    pub fn set(&mut self, current: f64, max: f64) {
        self.current = current;
        self.max = max;
    }

    pub fn ratio(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}

impl Mul<f64> for Hp {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            current: self.current * rhs,
            max: self.max * rhs,
        }
    }
}

impl Add for Hp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            current: (self.current + rhs.current).clamp(self.current, self.max),
            max: self.max,
        }
    }
}

impl Sub for Hp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            current: (self.current - rhs.current).clamp(0.0, self.current),
            max: self.max,
        }
    }
}

impl AddAssign for Hp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Hp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
