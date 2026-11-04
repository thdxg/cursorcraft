use crate::world::{
    World,
    components::{Coordinate, Direction, Health},
    entity::EntityKind,
};
use rand::{
    RngExt,
    seq::{IndexedRandom, index},
};

const ZOMBIE_RANDOM_SPAWN_CHANCE: f64 = 0.02;
const ZOMBIE_RANDOM_SPAWN_RADIUS: i32 = 50;
const ZOMBIE_IN_RADIUS_MAX: usize = 100;
const ZOMBIE_HEALTH_MAX: f64 = 50.0;
const ZOMBIE_GLYPH_UPDATE_PERIOD: usize = 5; // ticks
const ZOMBIE_SPEED: f64 = 0.03;
const ZOMBIE_ATTRACT_RADIUS: i32 = 100;

impl World {
    /// Zombie system including random spawn, glyph update, and behavior.
    pub(super) fn zombie(&mut self) {
        self.maybe_spawn();
        self.update_glyph();
        self.update_velocity();
    }

    fn can_spawn(&self) -> bool {
        let player_coord =
            self.player().and_then(|e| self.coordinate[e.id]).unwrap();
        let mut zombies_in_radius = 0;
        for x_offset in -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS
        {
            for y_offset in
                -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS
            {
                let offset = Coordinate::new(x_offset.into(), y_offset.into());
                let cell = (player_coord + offset).cell();
                if let Some(e) = self.occupancy.get(&cell)
                    && e.kind == EntityKind::Zombie
                {
                    zombies_in_radius += 1;
                }
            }
        }
        zombies_in_radius < ZOMBIE_IN_RADIUS_MAX
    }

    fn maybe_spawn(&mut self) {
        let mut rng = rand::rng();
        let spawn =
            rng.random_bool(ZOMBIE_RANDOM_SPAWN_CHANCE) && self.can_spawn();
        if spawn {
            // choose random coordinate within radius from player
            let x_offset = rng.random_range(
                -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS,
            );
            let y_offset = rng.random_range(
                -ZOMBIE_RANDOM_SPAWN_RADIUS..ZOMBIE_RANDOM_SPAWN_RADIUS,
            );
            let coord =
                self.player().and_then(|e| self.coordinate[e.id]).unwrap()
                    + Coordinate::new(f64::from(x_offset), f64::from(y_offset));

            // if cell already occupied, skip
            if self.occupancy.contains_key(&coord.cell()) {
                return;
            }

            // choose random health
            let health = Health::new(
                rng.random_range(0.0..ZOMBIE_HEALTH_MAX),
                ZOMBIE_HEALTH_MAX,
            );

            self.spawn_zombie(coord, health);
        }
    }

    fn update_glyph(&mut self) {
        static mut TICKS_SINCE_UPDATE: usize = 0;
        unsafe {
            if TICKS_SINCE_UPDATE < ZOMBIE_GLYPH_UPDATE_PERIOD {
                TICKS_SINCE_UPDATE += 1;
                return;
            }
        }
        let zombies = self
            .entities
            .iter()
            .filter(move |e| e.kind == EntityKind::Zombie);
        for e in zombies {
            let health = self.health[e.id].unwrap();
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "clamped to 0.0..=8.0"
            )]
            let dots = (health.ratio() * 8.0).ceil() as usize;
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

    fn update_velocity(&mut self) {
        let mut rng = rand::rng();
        let player_coord =
            self.player().and_then(|e| self.coordinate[e.id]).unwrap();
        let zombies = self
            .entities
            .iter()
            .filter(move |e| e.kind == EntityKind::Zombie);
        for e in zombies {
            let coord = self.coordinate[e.id]
                .expect("velocity must exist for a zombie");
            let distance = player_coord - coord;
            let attracted = distance.x.abs() <= ZOMBIE_ATTRACT_RADIUS.into()
                && distance.y.abs() <= ZOMBIE_ATTRACT_RADIUS.into();
            let vel = self.velocity[e.id]
                .as_mut()
                .expect("velocity must exist for a zombie");
            if attracted {
                vel.speed = ZOMBIE_SPEED;
                // follow the player 90% of the time
                vel.dir = if rng.random_bool(0.9) {
                    coord.dir_to(player_coord)
                } else {
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .to_owned()
                };
            } else {
                vel.speed = 0.0;
            }
        }
    }
}
