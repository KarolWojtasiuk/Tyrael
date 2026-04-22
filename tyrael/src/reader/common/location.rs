use crate::errors::ReadCharacterSaveError;
use crate::location::{GameAct, GameDifficulty, GameSaveLocation};

pub fn read_save_location_short(data: u16) -> Result<GameSaveLocation, ReadCharacterSaveError> {
    if data & 0xFF00 > 0 {
        return Err(ReadCharacterSaveError::InvalidSaveLocation(data as u32));
    }

    let act = match data & 0x000F {
        0x0000 => GameAct::Act1,
        0x0001 => GameAct::Act2,
        0x0002 => GameAct::Act3,
        0x0003 => GameAct::Act4,
        0x0004 => GameAct::Act5,
        _ => return Err(ReadCharacterSaveError::InvalidSaveLocation(data as u32)),
    };
    let difficulty = match data & 0x00F0 {
        0x0000 => GameDifficulty::Normal,
        0x0010 => GameDifficulty::Nightmare,
        0x0020 => GameDifficulty::Hell,
        _ => return Err(ReadCharacterSaveError::InvalidSaveLocation(data as u32)),
    };
    Ok(GameSaveLocation { difficulty, act })
}

pub fn read_save_location_long(data: [u8; 3]) -> Result<GameSaveLocation, ReadCharacterSaveError> {
    let difficulty_index = data.iter().position(|d| d & 0b1000_0000 > 0).ok_or(
        ReadCharacterSaveError::InvalidSaveLocation(u32::from_le_bytes([
            data[0], data[1], data[2], 0,
        ])),
    )?;
    let difficulty = match difficulty_index {
        0 => Ok(GameDifficulty::Normal),
        1 => Ok(GameDifficulty::Nightmare),
        2 => Ok(GameDifficulty::Hell),
        _ => Err(ReadCharacterSaveError::InvalidSaveLocation(
            u32::from_le_bytes([data[0], data[1], data[2], 0]),
        )),
    }?;
    let act = match data[difficulty_index] & 0b0111_1111 {
        0 => Ok(GameAct::Act1),
        1 => Ok(GameAct::Act2),
        2 => Ok(GameAct::Act3),
        3 => Ok(GameAct::Act4),
        4 => Ok(GameAct::Act5),
        _ => Err(ReadCharacterSaveError::InvalidSaveLocation(
            u32::from_le_bytes([data[0], data[1], data[2], 0]),
        )),
    }?;

    // TODO: tests
    // TODO: probably lossy conversion
    Ok(GameSaveLocation::new(difficulty, act))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_on_valid_save_location_short() {
        for (d, difficulty, act) in [
            (0x0000, GameDifficulty::Normal, GameAct::Act1),
            (0x0001, GameDifficulty::Normal, GameAct::Act2),
            (0x0002, GameDifficulty::Normal, GameAct::Act3),
            (0x0003, GameDifficulty::Normal, GameAct::Act4),
            (0x0004, GameDifficulty::Normal, GameAct::Act5),
            (0x0010, GameDifficulty::Nightmare, GameAct::Act1),
            (0x0011, GameDifficulty::Nightmare, GameAct::Act2),
            (0x0012, GameDifficulty::Nightmare, GameAct::Act3),
            (0x0013, GameDifficulty::Nightmare, GameAct::Act4),
            (0x0014, GameDifficulty::Nightmare, GameAct::Act5),
            (0x0020, GameDifficulty::Hell, GameAct::Act1),
            (0x0021, GameDifficulty::Hell, GameAct::Act2),
            (0x0022, GameDifficulty::Hell, GameAct::Act3),
            (0x0023, GameDifficulty::Hell, GameAct::Act4),
            (0x0024, GameDifficulty::Hell, GameAct::Act5),
        ] {
            assert_eq!(
                Ok(GameSaveLocation::new(difficulty, act)),
                read_save_location_short(d)
            );
        }
    }

    #[test]
    fn returns_err_on_invalid_save_location_short() {
        for d in [0x0005, 0x0030, 0x0017, 0x0029, 0x9900, 0xFFFF] {
            assert_eq!(
                Err(ReadCharacterSaveError::InvalidSaveLocation(d as u32)),
                read_save_location_short(d)
            )
        }
    }
}
