use derive_more::Display;

pub struct CharacterInfo {
    pub name: String,
    pub class: CharacterClass,
    pub status: CharacterStatus,
    pub progression: CharacterProgression,
    pub active_weapon: CharacterActiveWeaponSet,
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
