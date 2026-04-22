use std::slice::Iter;

use crate::CharacterSave;
use crate::character::CharacterData;
use crate::errors::ReadCharacterSaveError;
use crate::location::LocationData;
use crate::mercenary::MercenaryData;
use crate::reader::common::{ReaderExt, character, location, mercenary};

pub fn read_character_save(
    data: &mut Iter<u8>,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    let _filesize = data.read_u32()?;
    let _checksum = data.read_u32()?;

    Ok(CharacterSave {
        version,
        character: read_character_data(data)?,
        location: read_location_data(data)?,
        mercenary: Some(read_mercenary_data(data)?),
    })
}

fn read_character_data(data: &mut Iter<u8>) -> Result<CharacterData, ReadCharacterSaveError> {
    let active_weapon_set = character::read_active_weapon_set(data.read_u32()? as u8)?;
    let name = character::read_name(data.read_bytes::<16>()?)
        .map_err(ReadCharacterSaveError::InvalidCharacterName)?;
    let status = character::read_status(data.read_u8()?)?;
    let progression = character::read_progression(data.read_u8()?, status.expansion)?;

    {
        const EXPECTED: u16 = 0x0000;
        let actual = data.read_u16()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let class = character::read_class(data.read_u8()?)?;

    {
        const EXPECTED: u16 = 0x1E10;
        let actual = data.read_u16()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let menu_level = data.read_u8()?;

    {
        const EXPECTED: u32 = 0x00000000;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let last_played_at = Some(data.read_u32()?);

    {
        const EXPECTED: u32 = 0xFFFFFFFF;
        let actual = data.read_u32()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let skill_shortcuts = character::read_skill_shortcuts_long(data)?;
    let menu_appearance = character::read_menu_appearance(data.read_bytes::<32>()?)?;

    Ok(CharacterData {
        name,
        class,
        status,
        progression,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at,
    })
}

fn read_location_data(data: &mut Iter<u8>) -> Result<LocationData, ReadCharacterSaveError> {
    let save_location = location::read_save_location_long(data.read_bytes::<3>()?)?;
    let seed = data.read_u32()?;

    Ok(LocationData::new(seed, save_location))
}

fn read_mercenary_data(data: &mut Iter<u8>) -> Result<MercenaryData, ReadCharacterSaveError> {
    {
        const EXPECTED: u16 = 0x0000;
        let actual = data.read_u16()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_le_bytes().to_vec(),
                actual: actual.to_le_bytes().to_vec(),
            });
        }
    }

    let dead = mercenary::read_mercenary_dead(data.read_u16()?)?;
    let seed = data.read_u32()?;
    let name_id = data.read_u16()?;
    let kind = mercenary::read_mercenary_kind(data.read_u16()?)?;
    let experience = data.read_u32()?;

    {
        const EXPECTED: [u8; 144] = [0x00; 144];
        let actual = data.read_bytes::<144>()?;
        if actual != EXPECTED {
            return Err(ReadCharacterSaveError::InvalidMagicValue {
                expected: EXPECTED.to_vec(),
                actual: actual.to_vec(),
            });
        }
    }

    Ok(MercenaryData {
        seed,
        name_id,
        kind,
        experience,
        dead,
    })
}
