use crate::*;

pub const SACRIFICE_PALADIN_LEVEL_4: TestSave =
    include_save!(Classic, "1.00", Classic, Paladin, Normal, "SacrificeLevel4");

pub const TIGER_ASSASSIN_LEVEL_5: TestSave =
    include_save!(ClassicLoD, "1.13d", LoD, Assassin, Normal, "TigerLevel5");

pub fn saves() -> Vec<TestSave> {
    vec![SACRIFICE_PALADIN_LEVEL_4, TIGER_ASSASSIN_LEVEL_5]
}
