#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Health {
    current: f64,
    max: f64,
}

impl Health {
    pub fn new(current: f64, max: f64) -> Self {
        Self { current, max }
    }

    pub fn ratio(&self) -> f64 {
        if self.max == 0.0 {
            0.0
        } else {
            (self.current / self.max).clamp(0.0, 1.0)
        }
    }
}
