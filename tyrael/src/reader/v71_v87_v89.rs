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
    Ok(CharacterSave {
        version,
        character: read_character_info(data, version)?,
        progression: read_game_progression(data)?,
    })
}

fn read_character_info(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterInfo, ReadCharacterSaveError> {
    let name = character::read_name(data.read_bytes::<16>()?)
        .map_err(ReadCharacterSaveError::InvalidCharacterName)?;
    let status = character::read_status(data.read_u8()?)?;
    let game_completion = character::read_game_completion(data.read_u8()?, status.expansion)?;
    let active_weapon_set = character::read_active_weapon_set(data.read_u16()? as u8)?;

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

    let class = character::read_class(data.read_u16()? as u8)?;
    let menu_level = data.read_u16()? as u8;
    let menu_appearance = character::read_menu_appearance(data.read_bytes::<32>()?)?;
    let skill_shortcuts = character::read_skill_shortcuts_short(data)?;

    Ok(CharacterInfo {
        name,
        class,
        status,
        game_completion,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at: None,
    })
}

fn read_game_progression(data: &mut Iter<u8>) -> Result<GameProgression, ReadCharacterSaveError> {
    let save_location = progression::read_save_location_short(data.read_u16()?)?;
    Ok(GameProgression { save_location })
}
