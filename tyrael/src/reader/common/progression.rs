use crate::character::CharacterProgression;
use crate::errors::ReadCharacterSaveError;

pub fn read_character_progression(
    data: u8,
    expansion: bool,
) -> Result<CharacterProgression, ReadCharacterSaveError> {
    Ok(match expansion {
        false => match data {
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
                    progression: data,
                });
            }
        },
        true => match data {
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
                    progression: data,
                });
            }
        },
    })
}
