use crate::{
    game::Game,
    world::components::{Coordinate, Direction},
};

const PLAYER_SPEED: f64 = 1.0;

impl Game {
    // Moves player to a direction
    pub fn move_player(&mut self, dir: Direction) {
        let vel = self
            .world
            .player()
            .and_then(|e| self.world.velocity[e.id].as_mut())
            .unwrap();
        vel.dir = dir;
        vel.speed = PLAYER_SPEED;
    }

    pub fn move_player_to(&mut self, dest: Coordinate) {
        let coord = self
            .world
            .player()
            .and_then(|e| self.world.coordinate[e.id].as_mut())
            .unwrap();
        *coord = dest;
    }

    pub fn shoot(&mut self) {
        let vel = self
            .world
            .player()
            .and_then(|e| self.world.velocity[e.id])
            .unwrap();
        let coord = self
            .world
            .player()
            .and_then(|e| self.world.coordinate[e.id])
            .unwrap();
        self.world.spawn_bullet(coord, vel.dir);
    }

    // Spawns a block at player's coordinate and moves player to right
    pub fn place_block(&mut self) {
        let coord = self
            .world
            .player()
            .and_then(|e| self.world.coordinate[e.id])
            .unwrap();
        self.world.spawn_block(coord);
        self.move_player(Direction::Right);
    }

    pub fn move_camera(&mut self, dir: Direction) {
        self.camera.move_center(dir);
    }
}
