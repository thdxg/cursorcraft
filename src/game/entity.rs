mod bullet;
mod hp;
mod player;
mod zombie;

use super::Game;
use super::geometry::Pos;
pub use bullet::*;
pub use hp::*;
pub use player::*;
pub use zombie::*;

pub trait Entity {
    fn pos(&self) -> Pos;
    fn hp(&self) -> Hp;
    fn tick(&self, game: &Game) -> Option<Self>
    where
        Self: Sized;
}
