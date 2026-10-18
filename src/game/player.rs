pub struct Player {
    pub pos: (i64, i64), // (x, y)
    pub hp: Hp,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: (0, 0),
            hp: Hp {
                current: 100.0,
                max: 100.0,
            },
        }
    }
}

pub struct Hp {
    pub current: f64,
    pub max: f64,
}
