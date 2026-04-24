pub mod attribute;
pub mod character;
pub mod item;
pub mod location;
pub mod magic;
pub mod mercenary;
pub mod npc;
pub mod quest;
pub mod skill;
pub mod waypoint;

use std::slice::Iter;

use crate::U24F8;
use crate::errors::ReadCharacterSaveError;

pub trait ReaderExt {
    fn read_u8(&mut self) -> Result<u8, ReadCharacterSaveError>;
    fn read_u16(&mut self) -> Result<u16, ReadCharacterSaveError>;
    fn read_u32(&mut self) -> Result<u32, ReadCharacterSaveError>;
    fn read_u24f8(&mut self) -> Result<U24F8, ReadCharacterSaveError>;
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

    fn read_u24f8(&mut self) -> Result<U24F8, ReadCharacterSaveError> {
        Ok(U24F8::from_bits(self.read_u32()?))
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
