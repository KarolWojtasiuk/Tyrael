use bitter::LittleEndianReader;

use crate::U24F8;
use crate::attribute::{AttributeData, DynamicStats, PermanentStats, RankStats};
use crate::errors::{AttributeDataError, ReadCharacterSaveError};
use crate::reader::common::{ReaderExt, magic};

pub fn read_attributes_old(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<AttributeData, ReadCharacterSaveError> {
    let available_values = reader.u16()?;

    if version != 92 {
        magic::assert_magic_value_u8(0x00, reader.u8()?)?;
    }

    let mut permanent = PermanentStats::default();
    if available_values & (1 << 0) > 0 {
        permanent.strength = reader.u32()?;
    }
    if available_values & (1 << 1) > 0 {
        permanent.energy = reader.u32()?;
    }
    if available_values & (1 << 2) > 0 {
        permanent.dexterity = reader.u32()?;
    }
    if available_values & (1 << 3) > 0 {
        permanent.vitality = reader.u32()?;
    }
    if available_values & (1 << 4) > 0 {
        permanent.remaining_stat_points = reader.u32()?;
    }
    if available_values & (1 << 5) > 0 {
        permanent.remaining_skill_points = reader.u32()?;
    }

    let mut dynamic = DynamicStats::default();
    if available_values & (1 << 6) > 0 {
        dynamic.life = U24F8::from_bits(reader.u32()?);
    }
    if available_values & (1 << 7) > 0 {
        dynamic.max_life = U24F8::from_bits(reader.u32()?);
    }
    if available_values & (1 << 8) > 0 {
        dynamic.mana = U24F8::from_bits(reader.u32()?);
    }
    if available_values & (1 << 9) > 0 {
        dynamic.max_mana = U24F8::from_bits(reader.u32()?);
    }
    if available_values & (1 << 10) > 0 {
        dynamic.stamina = U24F8::from_bits(reader.u32()?);
    }
    if available_values & (1 << 11) > 0 {
        dynamic.max_stamina = U24F8::from_bits(reader.u32()?);
    }

    let mut rank = RankStats::default();
    if available_values & (1 << 12) > 0 {
        let level = reader.u32()?;
        if level > 99 {
            return Err(ReadCharacterSaveError::InvalidAttributeData(
                AttributeDataError::InvalidLevel(level),
            ));
        } else {
            rank.level = level as u8;
        }
    }
    if available_values & (1 << 13) > 0 {
        rank.experience = reader.u32()?;
    }
    if available_values & (1 << 14) > 0 {
        rank.gold = reader.u32()?;
    }
    if available_values & (1 << 15) > 0 {
        rank.stashed_gold = reader.u32()?;
    }

    Ok(AttributeData {
        permanent,
        dynamic,
        rank,
    })
}

pub fn read_attributes_new(
    reader: &mut LittleEndianReader,
) -> Result<AttributeData, ReadCharacterSaveError> {
    let mut attributes = AttributeData::default();

    loop {
        let attribute = reader.bits(9)?;
        match attribute {
            0 => attributes.permanent.strength = reader.bits(10)? as u32,
            1 => attributes.permanent.energy = reader.bits(10)? as u32,
            2 => attributes.permanent.dexterity = reader.bits(10)? as u32,
            3 => attributes.permanent.vitality = reader.bits(10)? as u32,
            4 => attributes.permanent.remaining_stat_points = reader.bits(10)? as u32,
            5 => attributes.permanent.remaining_skill_points = reader.bits(8)? as u32,
            6 => attributes.dynamic.life = U24F8::from_bits(reader.bits(21)? as u32),
            7 => attributes.dynamic.max_life = U24F8::from_bits(reader.bits(21)? as u32),
            8 => attributes.dynamic.mana = U24F8::from_bits(reader.bits(21)? as u32),
            9 => attributes.dynamic.max_mana = U24F8::from_bits(reader.bits(21)? as u32),
            10 => attributes.dynamic.stamina = U24F8::from_bits(reader.bits(21)? as u32),
            11 => attributes.dynamic.max_stamina = U24F8::from_bits(reader.bits(21)? as u32),
            12 => attributes.rank.level = reader.bits(7)? as u8,
            13 => attributes.rank.experience = reader.u32()?,
            14 => attributes.rank.gold = reader.bits(25)? as u32,
            15 => attributes.rank.stashed_gold = reader.bits(25)? as u32,
            511 => {
                reader.bits(reader.remainder().partial_bits() as u32)?;
                break;
            }
            _ => {
                return Err(ReadCharacterSaveError::InvalidAttributeData(
                    AttributeDataError::UnknownAttribute(attribute as u16),
                ));
            }
        }
    }

    Ok(attributes)
}
