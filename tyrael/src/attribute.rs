use derive_more::Constructor;

use crate::U24F8;

#[derive(Constructor, Debug, Default, PartialEq, Clone)]
pub struct AttributeData {
    pub permanent: PermanentStats,
    pub dynamic: DynamicStats,
    pub rank: RankStats,
}

#[derive(Constructor, Debug, Default, PartialEq, Clone)]
pub struct PermanentStats {
    pub strength: u32,
    pub dexterity: u32,
    pub vitality: u32,
    pub energy: u32,
    pub remaining_stat_points: u32,
    pub remaining_skill_points: u32,
}

#[derive(Constructor, Debug, Default, PartialEq, Clone)]
pub struct DynamicStats {
    pub life: U24F8,
    pub max_life: U24F8,
    pub mana: U24F8,
    pub max_mana: U24F8,
    pub stamina: U24F8,
    pub max_stamina: U24F8,
}

#[derive(Constructor, Debug, Default, PartialEq, Clone)]
pub struct RankStats {
    pub level: u8,
    pub experience: u32,
    pub gold: u32,
    pub stashed_gold: u32,
}
