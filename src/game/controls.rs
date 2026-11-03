use crate::{
    game::Game,
    world::components::{Coordinate, Direction},
};

const PLAYER_SPEED: f64 = 1.0;

impl Game {
    // Moves player to a direction
    pub fn move_player(&mut self, dir: Direction) {
        let player = self
            .world
            .player()
            .expect("player must exist in Playing status");
        let vel = self.world.velocity[player.id]
            .as_mut()
            .expect("velocity must exist for a player");
        vel.dir = dir;
        vel.speed = PLAYER_SPEED;
    }

    pub fn move_player_to(&mut self, dest: Coordinate) {
        let player = self
            .world
            .player()
            .expect("player must exist in Playing status");
        let coord = self.world.coordinate[player.id]
            .as_mut()
            .expect("coordinate must exist for a player");
        *coord = dest;
    }

    // Spawns a block at player's coordinate and moves player to right
    pub fn place_block(&mut self) {
        let player = self
            .world
            .player()
            .expect("player must exist in Playing status");
        let coord = self.world.coordinate[player.id]
            .expect("coordinate must exist for a player");
        self.world.spawn_block(coord);
        self.move_player(Direction::Right);
    }

    pub fn move_camera(&mut self, dir: Direction) {
        self.camera.move_center(dir);
    }
}
