use derive_more::{Constructor, Display};

pub struct CharacterInfo {
    pub name: String,
    pub class: CharacterClass,
    pub status: CharacterStatus,
    pub progression: CharacterProgression,
    pub active_weapon: CharacterActiveWeaponSet,
    pub menu_level: u8,
    pub last_played_at: Option<u32>,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum CharacterClass {
    Amazon,
    Sorceress,
    Necromancer,
    Paladin,
    Barbarian,
    Druid,
    Assassin,
    Warlock,
}

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct CharacterStatus {
    pub hardcore: bool,
    pub dead: bool,
    pub expansion: bool,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum CharacterProgression {
    None,
    NormalAndarielKilled,
    NormalDurielKilled,
    NormalMephistoKilled,
    NormalFinalBossKilled,
    NightmareAndarielKilled,
    NightmareDurielKilled,
    NightmareMephistoKilled,
    NightmareFinalBossKilled,
    HellAndarielKilled,
    HellDurielKilled,
    HellMephistoKilled,
    HellFinalBossKilled,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum CharacterActiveWeaponSet {
    WeaponSet1,
    WeaponSet2,
}
