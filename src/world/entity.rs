use std::fmt::Debug;

const DEFAULT_ARENA_SIZE: usize = 1024;

pub(super) struct Arena {
    pub size: usize,
    free: Vec<usize>,
}

impl Arena {
    /// Returns the index of a free slot.
    ///
    /// If there is no free slot, grows the arena by 1.
    pub fn alloc(&mut self) -> usize {
        if let Some(i) = self.free.pop() {
            i
        } else {
            self.size += 1;
            self.size - 1
        }
    }

    pub fn free(&mut self, i: usize) {
        self.free.push(i);
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            size: DEFAULT_ARENA_SIZE,
            free: (0..DEFAULT_ARENA_SIZE).collect(),
        }
    }
}

impl Debug for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("size", &self.size)
            .field("free", &self.free.len())
            .finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Entity {
    pub id: usize,
    pub kind: EntityKind,
}

impl Entity {
    pub fn new(id: usize, kind: EntityKind) -> Self {
        Self { id, kind }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum EntityKind {
    Player,
    Zombie,
    Block,
}
