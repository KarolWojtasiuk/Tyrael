use std::slice::Iter;

use crate::errors::{MercenaryDataError, ReadCharacterSaveError};
use crate::mercenary::MercenaryKind;
use crate::reader::common::ReaderExt;

pub fn read_mercenary_dead(data: &mut Iter<u8>) -> Result<bool, ReadCharacterSaveError> {
    let data = data.read_u16()?;
    match data {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(MercenaryDataError::InvalidDeadFlag(data).into()),
    }
}

pub fn read_mercenary_kind(data: &mut Iter<u8>) -> Result<MercenaryKind, ReadCharacterSaveError> {
    let data = data.read_u16()?;
    match data {
        0 => Ok(MercenaryKind::None),
        1 => Ok(MercenaryKind::Act1),
        // TODO
        _ => Err(MercenaryDataError::InvalidKind(data).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_on_valid_mercenary_dead_flag() {
        for (d, dead) in [(0x0000, false), (0x0001, true)] {
            assert_eq!(
                Ok(dead),
                read_mercenary_dead(&mut u16::to_le_bytes(d).iter())
            );
        }
    }

    #[test]
    fn returns_err_on_invalid_mercenary_dead_flag() {
        for d in [0x0002, 0xFF00, 0x0F01] {
            assert!(read_mercenary_dead(&mut u16::to_le_bytes(d).iter()).is_err());
        }
    }

    #[test]
    fn returns_ok_on_valid_mercenary_kind() {
        for (d, kind) in [(0x0000, MercenaryKind::None), (0x0001, MercenaryKind::Act1)] {
            assert_eq!(
                Ok(kind),
                read_mercenary_kind(&mut u16::to_le_bytes(d).iter())
            );
        }
    }

    #[test]
    fn returns_err_on_invalid_mercenary_kind() {
        for d in [0x0002, 0xFF00, 0x0F01] {
            assert!(read_mercenary_kind(&mut u16::to_le_bytes(d).iter()).is_err());
        }
    }
}
