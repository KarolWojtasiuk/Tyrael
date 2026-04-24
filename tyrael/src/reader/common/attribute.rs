use std::slice::Iter;

use crate::attribute::{AttributeData, DynamicStats, PermanentStats, RankStats};
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::{ReaderExt, magic};

pub fn read_attributes_old(data: &mut Iter<u8>) -> Result<AttributeData, ReadCharacterSaveError> {
    let available_values = data.read_u16()?;
    magic::assert_magic_value_u8(0x00, data.read_u8()?)?;

    let mut permanent = PermanentStats::default();
    if available_values & (1 << 0) > 0 {
        permanent.strength = data.read_u32()?;
    }
    if available_values & (1 << 1) > 0 {
        permanent.energy = data.read_u32()?;
    }
    if available_values & (1 << 2) > 0 {
        permanent.dexterity = data.read_u32()?;
    }
    if available_values & (1 << 3) > 0 {
        permanent.vitality = data.read_u32()?;
    }
    if available_values & (1 << 4) > 0 {
        permanent.remaining_stat_points = data.read_u32()?;
    }
    if available_values & (1 << 5) > 0 {
        permanent.remaining_skill_points = data.read_u32()?;
    }

    let mut dynamic = DynamicStats::default();
    if available_values & (1 << 6) > 0 {
        dynamic.life = data.read_u24f8()?;
    }
    if available_values & (1 << 7) > 0 {
        dynamic.max_life = data.read_u24f8()?;
    }
    if available_values & (1 << 8) > 0 {
        dynamic.mana = data.read_u24f8()?;
    }
    if available_values & (1 << 9) > 0 {
        dynamic.max_mana = data.read_u24f8()?;
    }
    if available_values & (1 << 10) > 0 {
        dynamic.stamina = data.read_u24f8()?;
    }
    if available_values & (1 << 11) > 0 {
        dynamic.max_stamina = data.read_u24f8()?;
    }

    let mut rank = RankStats::default();
    if available_values & (1 << 12) > 0 {
        rank.level = data.read_u32()?;
    }
    if available_values & (1 << 13) > 0 {
        rank.experience = data.read_u32()?;
    }
    if available_values & (1 << 14) > 0 {
        rank.gold = data.read_u32()?;
    }
    if available_values & (1 << 15) > 0 {
        rank.stashed_gold = data.read_u32()?;
    }

    Ok(AttributeData {
        permanent,
        dynamic,
        rank,
    })
}
