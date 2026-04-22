use derive_more::{Constructor, Display};

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct QuestData {
    // pub normal: DifficultyQuests,
    // pub nightmare: DifficultyQuests,
    // pub hell: DifficultyQuests,
}

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct DifficultyQuests {
    pub metadata: DifficultyMetadata,
    pub act1: [QuestStatus; 6],
    pub act2: [QuestStatus; 6],
    pub act3: [QuestStatus; 6],
    pub act4: [QuestStatus; 3],
    pub act5: Option<[QuestStatus; 6]>,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum QuestStatus {}

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct DifficultyMetadata {}
