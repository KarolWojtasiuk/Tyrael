use derive_more::{Constructor, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct GameProgression {
    pub save_location: GameSaveLocation,
}

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct GameSaveLocation {
    pub difficulty: GameDifficulty,
    pub act: GameAct,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum GameDifficulty {
    Normal,
    Nightmare,
    Hell,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum GameAct {
    Act1,
    Act2,
    Act3,
    Act4,
    Act5,
}
