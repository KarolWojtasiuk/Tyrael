use derive_more::{Constructor, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterData {
    pub name: String,
    pub class: CharacterClass,
    pub status: CharacterStatus,
    pub progression: CharacterProgression,
    pub active_weapon_set: CharacterActiveWeaponSet,
    pub menu_level: u8,
    pub menu_appearance: CharacterMenuAppearance,
    pub skill_shortcuts: CharacterSkillShortcuts,
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CharacterMenuAppearance(pub [u8; 32]);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharacterSkillShortcuts {
    Short {
        keyboard: [u16; 8],
        lmb: u8,
        rmb: u8,
    },
    Long {
        keyboard: [u32; 16],
        lmb: u32,
        rmb: u32,
        lmb_switch: u32,
        rmb_switch: u32,
    },
}
