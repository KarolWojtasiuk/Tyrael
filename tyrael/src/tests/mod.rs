pub mod saves;

mod starters;

#[allow(unused)]
pub struct TestSave {
    pub bytes: &'static [u8],
    pub filename: &'static str,
    pub game: TestSaveGame,
    pub version: &'static str,
    pub expansion: TestSaveExpansion,
    pub class: TestSaveClass,
    pub name: &'static str,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveGame {
    Classic,
    ClassicLoD,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveExpansion {
    Classic,
    LoD,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum TestSaveClass {
    Amazon,
    Barbarian,
    Paladin,
    Necromancer,
    Sorceress,
    Assassin,
    Druid,
}
