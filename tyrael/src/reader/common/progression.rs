use std::slice::Iter;

use crate::character::CharacterProgression;
use crate::errors::ReadCharacterSaveError;
use crate::reader::common::ReaderExt;

pub fn read_character_progression(
    data: &mut Iter<u8>,
    expansion: bool,
) -> Result<CharacterProgression, ReadCharacterSaveError> {
    let progression = data.read_u8()?;

    Ok(match expansion {
        false => match progression {
            0 => CharacterProgression::None,
            1 => CharacterProgression::NormalAndarielKilled,
            2 => CharacterProgression::NormalDurielKilled,
            3 => CharacterProgression::NormalMephistoKilled,
            4 => CharacterProgression::NormalFinalBossKilled,
            5 => CharacterProgression::NightmareAndarielKilled,
            6 => CharacterProgression::NightmareDurielKilled,
            7 => CharacterProgression::NightmareMephistoKilled,
            8 => CharacterProgression::NightmareFinalBossKilled,
            9 => CharacterProgression::HellAndarielKilled,
            10 => CharacterProgression::HellDurielKilled,
            11 => CharacterProgression::HellMephistoKilled,
            12 => CharacterProgression::HellFinalBossKilled,
            _ => {
                return Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion,
                    progression,
                });
            }
        },
        true => match progression {
            0 => CharacterProgression::None,
            1 => CharacterProgression::NormalAndarielKilled,
            2 => CharacterProgression::NormalDurielKilled,
            3 => CharacterProgression::NormalMephistoKilled,
            5 => CharacterProgression::NormalFinalBossKilled,
            6 => CharacterProgression::NightmareAndarielKilled,
            7 => CharacterProgression::NightmareDurielKilled,
            8 => CharacterProgression::NightmareMephistoKilled,
            10 => CharacterProgression::NightmareFinalBossKilled,
            11 => CharacterProgression::HellAndarielKilled,
            12 => CharacterProgression::HellDurielKilled,
            13 => CharacterProgression::HellMephistoKilled,
            15 => CharacterProgression::HellFinalBossKilled,
            _ => {
                return Err(ReadCharacterSaveError::InvalidCharacterProgression {
                    expansion,
                    progression,
                });
            }
        },
    })
}
