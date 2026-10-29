use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum GameStatus {
    Playing(GameMode),
    Paused,
    Finished,
    Exited,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Playing(GameMode::default())
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Insert,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameMode::Normal => f.write_str("NORMAL"),
            GameMode::Insert => f.write_str("INSERT"),
        }
    }
}
