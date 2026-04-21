use crate::*;

pub const SACRIFICE_LEVEL_4: TestSave =
    include_save!(Classic, "1.00", Classic, Paladin, Normal, "SacrificeLevel4");

pub fn saves() -> Vec<TestSave> {
    vec![SACRIFICE_LEVEL_4]
}
