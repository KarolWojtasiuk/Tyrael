use std::slice::Iter;

use crate::CharacterSave;
use crate::character::CharacterInfo;
use crate::errors::ReadCharacterSaveError;
use crate::progression::GameProgression;
use crate::reader::common::{ReaderExt, character, progression};

pub fn read_character_save(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    let _filesize = data.read_u32()?;
    let _checksum = data.read_u32()?;

    Ok(CharacterSave {
        version,
        character: read_character_info(data)?,
        progression: read_game_progression(data)?,
    })
}

fn read_character_info(data: &mut Iter<u8>) -> Result<CharacterInfo, ReadCharacterSaveError> {
    let active_weapon_set = character::read_active_weapon_set(data.read_u32()? as u8)?;
    let name = character::read_name(data.read_bytes::<16>()?)
        .map_err(ReadCharacterSaveError::InvalidCharacterName)?;
    let status = character::read_status(data.read_u8()?)?;
    let game_completion = character::read_game_completion(data.read_u8()?, status.expansion)?;

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

    let class = character::read_class(data.read_u8()?)?;

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

    let skill_shortcuts = character::read_skill_shortcuts_long(data)?;
    let menu_appearance = character::read_menu_appearance(data.read_bytes::<32>()?)?;

    Ok(CharacterInfo {
        name,
        class,
        status,
        game_completion,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at,
    })
}

fn read_game_progression(data: &mut Iter<u8>) -> Result<GameProgression, ReadCharacterSaveError> {
    let save_location = progression::read_save_location_long(data.read_bytes::<3>()?)?;
    Ok(GameProgression { save_location })
}
