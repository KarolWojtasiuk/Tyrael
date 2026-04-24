use crate::errors::{MercenaryDataError, ReadCharacterSaveError};
use crate::mercenary::MercenaryKind;

pub fn read_mercenary_dead(data: u16) -> Result<bool, ReadCharacterSaveError> {
    match data {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(MercenaryDataError::InvalidDeadFlag(data).into()),
    }
}

pub fn read_mercenary_kind(data: u16) -> Result<MercenaryKind, ReadCharacterSaveError> {
    match data {
        0 => Ok(MercenaryKind::None),
        1 => Ok(MercenaryKind::Act1),
        // TODO
        _ => Err(MercenaryDataError::InvalidKind(data).into()),
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(0x0000, false)]
    #[test_case(0x0001, true)]
    fn returns_ok_on_valid_mercenary_dead_flag(data: u16, dead: bool) {
        assert_eq!(Ok(dead), read_mercenary_dead(data));
    }

    #[test_case(0x0002)]
    #[test_case(0xFF00)]
    #[test_case(0x0F01)]
    fn returns_err_on_invalid_mercenary_dead_flag(data: u16) {
        assert!(read_mercenary_dead(data).is_err());
    }

    #[test_case(0x0000, MercenaryKind::None)]
    #[test_case(0x0001, MercenaryKind::Act1)]
    fn returns_ok_on_valid_mercenary_kind(data: u16, kind: MercenaryKind) {
        assert_eq!(Ok(kind), read_mercenary_kind(data));
    }

    #[test_case(0x0002)]
    #[test_case(0xFF00)]
    #[test_case(0x0F01)]
    fn returns_err_on_invalid_mercenary_kind(data: u16) {
        assert!(read_mercenary_kind(data).is_err());
    }
}
