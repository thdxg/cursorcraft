use std::any::Any;

use crate::game::geometry::Coord;

use super::{Entity, Hp};

#[derive(Copy, Clone)]
pub struct Block {
    pub kind: BlockKind,
    coord: Coord,
    hp: Hp,
}

impl Block {
    pub fn new(kind: BlockKind, coord: Coord) -> Self {
        Self {
            kind,
            coord,
            hp: kind.initial_hp(),
        }
    }
}

impl Entity for Block {
    fn coord(&self) -> Coord {
        self.coord
    }

    fn hp(&self) -> Hp {
        self.hp
    }

    fn tick(&self, _: &mut crate::game::Game) -> Option<Self> {
        Some(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Copy, Clone)]
pub enum BlockKind {
    Wall,
    Trap,
}

impl BlockKind {
    fn initial_hp(self) -> Hp {
        match self {
            BlockKind::Wall => Hp::new(30.0, 30.0),
            BlockKind::Trap => Hp::new(20.0, 20.0),
        }
    }
}

impl From<BlockKind> for char {
    fn from(val: BlockKind) -> Self {
        match val {
            BlockKind::Wall => '#',
            BlockKind::Trap => 'x',
        }
    }
}

impl TryFrom<char> for BlockKind {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            'x' => Ok(Self::Trap),
            _ => Err("no block kind represents this character".to_string()),
        }
    }
}
