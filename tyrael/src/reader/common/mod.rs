mod active_weapon;
mod class;
mod name;
mod progression;
mod status;

use std::slice::Iter;

pub use active_weapon::read_character_active_weapon;
pub use class::read_character_class;
pub use name::read_character_name;
pub use progression::read_character_progression;
pub use status::read_character_status;

use crate::errors::ReadCharacterSaveError;

pub trait ReaderExt {
    fn read_u8(&mut self) -> Result<u8, ReadCharacterSaveError>;
    fn read_u16(&mut self) -> Result<u16, ReadCharacterSaveError>;
    fn read_u32(&mut self) -> Result<u32, ReadCharacterSaveError>;
    fn read_bytes<const B: usize>(&mut self) -> Result<[u8; B], ReadCharacterSaveError>;
}

impl<'a> ReaderExt for Iter<'a, u8> {
    fn read_u8(&mut self) -> Result<u8, ReadCharacterSaveError> {
        match self.next() {
            Some(d) => Ok(*d),
            None => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn read_u16(&mut self) -> Result<u16, ReadCharacterSaveError> {
        match self.next_chunk::<2>() {
            Ok(d) => Ok(u16::from_le_bytes([*d[0], *d[1]])),
            Err(_) => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn read_u32(&mut self) -> Result<u32, ReadCharacterSaveError> {
        match self.next_chunk::<4>() {
            Ok(d) => Ok(u32::from_le_bytes([*d[0], *d[1], *d[2], *d[3]])),
            Err(_) => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }

    fn read_bytes<const B: usize>(&mut self) -> Result<[u8; B], ReadCharacterSaveError> {
        match self.next_chunk::<B>() {
            Ok(d) => {
                let mut result = [0u8; B];
                for i in 0..B {
                    result[i] = *d[i];
                }
                Ok(result)
            }
            Err(_) => Err(ReadCharacterSaveError::UnexpectedEOF),
        }
    }
}
