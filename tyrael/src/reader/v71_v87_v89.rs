use std::slice::Iter;

use crate::CharacterSave;
use crate::character::CharacterData;
use crate::errors::ReadCharacterSaveError;
use crate::location::LocationData;
use crate::quest::QuestData;
use crate::reader::common::{ReaderExt, character, location, quest};

pub fn read_character_save(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    Ok(CharacterSave {
        version,
        character: read_character_data(data, version)?,
        location: read_location_data(data)?,
        mercenary: None,
        quests: read_quest_data(data)?,
    })
}

fn read_character_data(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterData, ReadCharacterSaveError> {
    let name = character::read_name(data.read_bytes::<16>()?)
        .map_err(ReadCharacterSaveError::InvalidCharacterName)?;
    let status = character::read_status(data.read_u8()?)?;
    let progression = character::read_progression(data.read_u8()?, status.expansion)?;
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
                expected: expected.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    {
        const EXPECTED: u32 = 0x00820010;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let class = character::read_class(data.read_u16()? as u8)?;
    let menu_level = data.read_u16()? as u8;
    let menu_appearance = character::read_menu_appearance(data.read_bytes::<32>()?)?;
    let skill_shortcuts = character::read_skill_shortcuts_short(data)?;

    Ok(CharacterData {
        name,
        class,
        status,
        progression,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at: None,
    })
}

fn read_location_data(data: &mut Iter<u8>) -> Result<LocationData, ReadCharacterSaveError> {
    let save_location = location::read_save_location_short(data.read_u16()?)?;

    {
        const EXPECTED: [u8; 36] = [0; 36];
        let actual = data.read_bytes::<36>()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_vec(),
                actual: actual.to_vec(),
            });
        }
    }

    let seed = data.read_u32()?;

    Ok(LocationData::new(seed, save_location))
}

fn read_quest_data(data: &mut Iter<u8>) -> Result<QuestData, ReadCharacterSaveError> {
    quest::read_woo_header(data)?;

    Ok(QuestData {})
}
