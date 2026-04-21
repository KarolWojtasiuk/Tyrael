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
    let active_weapon = common::read_character_active_weapon_u32(data)?;
    let name = common::read_character_name(data)?;
    let status = common::read_character_status(data)?;
    let progression = common::read_character_progression(data, status.expansion)?;

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

    let class = common::read_character_class(data)?;

    Ok(CharacterInfo {
        name,
        class,
        status,
        progression,
        active_weapon,
    })
}
