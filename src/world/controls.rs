use crate::world::{World, components::Direction};

const PLAYER_SPEED: f64 = 1.0;

impl World {
    // Moves player to a direction
    pub fn move_player(&mut self, dir: Direction) {
        let player =
            self.player().expect("player must exist in Playing status");
        let vel = self.velocity[player.id]
            .as_mut()
            .expect("velocity must exist for a player");
        vel.dir = dir;
        vel.speed = PLAYER_SPEED;
    }

    // Spawns a block at player's coordinate and moves player to right
    pub fn place_block(&mut self) {
        let player =
            self.player().expect("player must exist in Playing status");
        let coord = self.coordinate[player.id]
            .expect("coordinate must exist for a player");
        self.spawn_block(coord);
        self.move_player(Direction::Right);
    }
}
