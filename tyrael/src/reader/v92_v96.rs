use std::slice::Iter;

use super::common;
use super::common::ReaderExt;
use crate::CharacterSave;
use crate::character::CharacterInfo;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_save(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    let _filesize = data.read_u32()?;
    let _checksum = data.read_u32()?;

    Ok(CharacterSave {
        version,
        character: read_character_info(data)?,
    })
}

pub fn read_character_info(data: &mut Iter<u8>) -> Result<CharacterInfo, ReadCharacterSaveError> {
    let active_weapon = common::read_character_active_weapon(data.read_u32()? as u8)?;
    let name = common::read_character_name(data.read_bytes::<16>()?)
        .map_err(ReadCharacterSaveError::InvalidCharacterName)?;
    let status = common::read_character_status(data.read_u8()?)?;
    let progression = common::read_character_progression(data.read_u8()?, status.expansion)?;

    {
        const EXPECTED: u16 = 0x0000;
        let actual = data.read_u16()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED as u32,
                actual: actual as u32,
            });
        }
    }

    let class = common::read_character_class(data.read_u8()?)?;

    {
        const EXPECTED: u16 = 0x1E10;
        let actual = data.read_u16()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED as u32,
                actual: actual as u32,
            });
        }
    }

    let menu_level = data.read_u8()?;

    {
        const EXPECTED: u32 = 0x00000000;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED,
                actual,
            });
        }
    }

    let last_played_at = Some(data.read_u32()?);

    {
        const EXPECTED: u32 = 0xFFFFFFFF;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED,
                actual,
            });
        }
    }

    Ok(CharacterInfo {
        name,
        class,
        status,
        progression,
        active_weapon,
        menu_level,
        last_played_at,
    })
}
