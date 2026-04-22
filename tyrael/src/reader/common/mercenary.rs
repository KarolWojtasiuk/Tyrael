use crate::errors::ReadCharacterSaveError;
use crate::mercenary::MercenaryKind;

pub fn read_mercenary_dead(data: u16) -> Result<bool, ReadCharacterSaveError> {
    match data {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ReadCharacterSaveError::InvalidMercenaryDeadFlag(data)),
    }
}

pub fn read_mercenary_kind(data: u16) -> Result<MercenaryKind, ReadCharacterSaveError> {
    match data {
        0 => Ok(MercenaryKind::None),
        1 => Ok(MercenaryKind::Act1),
        // TODO
        _ => Err(ReadCharacterSaveError::InvalidMercenaryKind(data)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_on_valid_mercenary_dead_flag() {
        for (d, dead) in [(0x0000, false), (0x0001, true)] {
            assert_eq!(Ok(dead), read_mercenary_dead(d));
        }
    }

    #[test]
    fn returns_err_on_invalid_mercenary_dead_flag() {
        for d in [0x0002, 0xFF00, 0x0F01] {
            assert!(read_mercenary_dead(d).is_err());
        }
    }

    #[test]
    fn returns_ok_on_valid_mercenary_kind() {
        for (d, kind) in [(0x0000, MercenaryKind::None), (0x0001, MercenaryKind::Act1)] {
            assert_eq!(Ok(kind), read_mercenary_kind(d));
        }
    }

    #[test]
    fn returns_err_on_invalid_mercenary_kind() {
        for d in [0x0002, 0xFF00, 0x0F01] {
            assert!(read_mercenary_kind(d).is_err());
        }
    }
}
