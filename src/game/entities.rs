use super::geometry::{Dir, Pos};

pub struct Hp {
    pub current: f64,
    pub max: f64,
}

impl Hp {
    pub fn new(max: f64) -> Self {
        Self { current: max, max }
    }

    pub fn ratio(&self) -> f64 {
        self.current / self.max
    }
}

pub struct Player {
    pub pos: Pos,
    pub hp: Hp,
    loaded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Pos::default(),
            hp: Hp::new(100.0),
            loaded: false,
        }
    }
}

impl Player {
    pub fn step(&mut self, dir: Dir) {
        self.pos.step(dir);
    }

    pub fn shoot(&mut self, dir: Dir) {
        todo!()
    }
}

pub struct Enemy {
    pub pos: Pos,
    pub hp: Hp,
    pub dir: Dir,
}

impl Enemy {
    pub fn new(pos: Pos, hp: Hp, dir: Dir) -> Self {
        Self { pos, hp, dir }
    }
}

pub enum Shooter {
    Player,
    Enemy,
}

pub struct Bullet {
    pub pos: Pos,
    pub dir: Dir,
    pub is_player: bool,
}

impl Bullet {
    pub fn new(pos: Pos, dir: Dir, is_player: bool) -> Self {
        Self {
            pos,
            dir,
            is_player,
        }
    }
}
