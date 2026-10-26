mod block;
mod bullet;
mod hp;
mod player;
mod zombie;

use std::any::Any;

use super::Game;
use super::geometry::Coord;
pub use block::*;
pub use bullet::*;
pub use hp::*;
pub use player::*;
pub use zombie::*;

pub trait Entity {
    fn coord(&self) -> Coord;
    fn hp(&self) -> Hp;
    fn tick(&self, game: &mut Game) -> Option<Self>
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
}
