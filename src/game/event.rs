use super::{Entity, Block};
use super::geometry::Dir;

#[derive(Copy, Clone)]
pub enum Event<'a> {
    MovePlayer(Dir),
    PlaceBlock(Block),
    Damage {
        target: &'a dyn Entity,
        amount: super::Hp,
    },
}
