use std::slice::Iter;

use super::common;
use crate::CharacterSave;
use crate::character::CharacterInfo;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_character_save(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    Ok(CharacterSave {
        version,
        character: read_character_info(data, version)?,
    })
}

pub fn read_character_info(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterInfo, ReadCharacterSaveError> {
    let name = common::read_character_name(data)?;
    let status = common::read_character_status(data)?;
    let progression = common::read_character_progression(data, status.expansion)?;
    let active_weapon = common::read_character_active_weapon_u16(data)?;

    {
        let expected = match version {
            71 => 0x00DD,
            87 | 89 => 0x013F,
            _ => unreachable!("unexpected version '{}' for v71_v87_v89 reader", version),
        };
        let actual = data.read_u16()?;
        if actual != expected {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: expected as u32,
                actual: actual as u32,
            });
        }
    }

    {
        const EXPECTED: u32 = 0x00820010;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED,
                actual,
            });
        }
    }

    let class = common::read_character_class(data)?;

    Ok(CharacterInfo {
        name,
        class,
        status,
        progression,
        active_weapon,
    })
}
