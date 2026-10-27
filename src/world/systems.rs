use rand::{RngExt, seq::index};

use crate::world::{
    components::{Coordinate, Health},
    entity::Entity,
};

use super::{World, entity::EntityKind};

const ZOMBIE_RANDOM_SPAWN_CHANCE: f64 = 0.02;
const ZOMBIE_RANDOM_SPAWN_RADIUS: f64 = 50.0;
const ZOMBIE_HEALTH_MAX: f64 = 50.0;
const ZOMBIE_GLYPH_UPDATE_FREQ: usize = 5; // ticks
const ZOMBIE_SPEED: f64 = 0.015;
const ZOMBIE_ATTRACT_RADIUS: f64 = 30.0;

impl World {
    /// Update entity coordinates
    pub(super) fn sys_movement(&mut self) {
        for e in &self.entities {
            if let (Some(vel), Some(coord)) =
                (self.velocity[e.id].as_mut(), self.coordinate[e.id].as_mut())
            {
                coord.step(*vel);
                // reset player velocity after one move
                if e.kind == EntityKind::Player {
                    vel.speed = 0.0;
                }
            }
        }
    }

    pub(super) fn sys_zombie_random_spawn(&mut self) {
        let mut rng = rand::rng();
        let spawn = rng.random_bool(ZOMBIE_RANDOM_SPAWN_CHANCE);
        if spawn {
            // choose random coordinate within radius from player
            let player = self
                .player()
                .expect("player must exist when systems are called");
            let x_offset = rng.random_range(
                -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS,
            );
            let y_offset = rng.random_range(
                -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS,
            );
            let coord = self.coordinate[player.id]
                .expect("coordinate must exist for a player")
                + Coordinate::new(x_offset, y_offset);

            // choose random health
            let health = Health::new(
                rng.random_range(0.0..ZOMBIE_HEALTH_MAX),
                ZOMBIE_HEALTH_MAX,
            );

            self.spawn_zombie(coord, health);
        }
    }

    pub(super) fn sys_zombie_glyph(&mut self) {
        static mut TICKS_SINCE_UPDATE: usize = 0;
        unsafe {
            if TICKS_SINCE_UPDATE < ZOMBIE_GLYPH_UPDATE_FREQ {
                TICKS_SINCE_UPDATE += 1;
                return;
            }
        }
        let zombies = self
            .entities
            .iter()
            .filter(move |e| e.kind == EntityKind::Zombie);
        for e in zombies {
            let health =
                self.health[e.id].expect("health must exist for a zombie");
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "clamped to 0.0..=8.0"
            )]
            let dots = (health.ratio() * 8.0) as usize;
            let mask = index::sample(&mut rand::rng(), 8, dots)
                .into_iter()
                .fold(0x0, |mask, bit| mask | 1 << bit);
            let glyph =
                char::from_u32(0x2800 /* braille start unicode */ + mask)
                    .unwrap();
            self.glyph[e.id] = Some(glyph);
            unsafe {
                TICKS_SINCE_UPDATE = 0;
            }
        }
    }

    pub(super) fn sys_zombie_ai(&mut self) {
        let player = self
            .player()
            .expect("player must exist when systems are called");
        let player_coord = self.coordinate[player.id]
            .expect("coordinate must exist for a player");
        let zombies = self
            .entities
            .iter()
            .filter(move |e| e.kind == EntityKind::Zombie);
        for e in zombies {
            let coord = self.coordinate[e.id]
                .expect("velocity must exist for a zombie");
            let distance = player_coord - coord;
            let attracted = distance.x.abs() <= ZOMBIE_ATTRACT_RADIUS
                && distance.y.abs() <= ZOMBIE_ATTRACT_RADIUS;
            let vel = self.velocity[e.id]
                .as_mut()
                .expect("velocity must exist for a zombie");
            if attracted {
                vel.speed = ZOMBIE_SPEED;
                vel.dir = coord.dir_to(player_coord);
            } else {
                vel.speed = 0.0;
            }
        }
    }

    /// Free dead entities
    pub(super) fn sys_cleanup(&mut self) {
        let to_free: Vec<Entity> = self
            .entities
            .iter()
            .filter(|e| self.health[e.id].is_some_and(|h| !h.is_alive()))
            .copied()
            .collect();

        for e in to_free {
            self.free_entity(e);
        }
    }
}
