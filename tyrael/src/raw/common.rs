use bitter::{BitReader, LittleEndianReader};

use crate::errors::ReadRawSaveDataError;

pub trait EofExt<T> {
    fn ok_or_eof(self) -> Result<T, ReadRawSaveDataError>;
}

impl<T> EofExt<T> for Option<T> {
    fn ok_or_eof(self) -> Result<T, ReadRawSaveDataError> {
        self.ok_or(ReadRawSaveDataError::UnexpectedEOF)
    }
}

pub trait ReaderExt {
    fn read_array<const S: usize>(&mut self) -> Result<[u8; S], ReadRawSaveDataError>;
    fn assert_section_header_u16(&mut self, expected: u16) -> Result<(), ReadRawSaveDataError>;
    fn assert_section_header_u32(&mut self, expected: u32) -> Result<(), ReadRawSaveDataError>;
}

impl ReaderExt for LittleEndianReader<'_> {
    fn read_array<const S: usize>(&mut self) -> Result<[u8; S], ReadRawSaveDataError> {
        let mut data = [0u8; S];
        if !self.read_bytes(&mut data) {
            Err(ReadRawSaveDataError::UnexpectedEOF)
        } else {
            Ok(data)
        }
    }

    fn assert_section_header_u16(&mut self, expected: u16) -> Result<(), ReadRawSaveDataError> {
        let actual = self.read_u16().ok_or_eof()?;
        if actual != expected {
            Err(ReadRawSaveDataError::MismatchedHeader {
                expected: expected as u32,
                actual: actual as u32,
            })
        } else {
            Ok(())
        }
    }

    fn assert_section_header_u32(&mut self, expected: u32) -> Result<(), ReadRawSaveDataError> {
        let actual = self.read_u32().ok_or_eof()?;
        if actual != expected {
            Err(ReadRawSaveDataError::MismatchedHeader { expected, actual })
        } else {
            Ok(())
        }
    }
}

pub fn read_attributes_v71_v87_v89(
    reader: &mut LittleEndianReader,
) -> Result<Vec<u8>, ReadRawSaveDataError> {
    read_attributes_v71_v87_v89_v92(reader, false)
}

pub fn read_attributes_v92(
    reader: &mut LittleEndianReader,
) -> Result<Vec<u8>, ReadRawSaveDataError> {
    read_attributes_v71_v87_v89_v92(reader, true)
}

pub fn read_attributes_v71_v87_v89_v92(
    reader: &mut LittleEndianReader,
    is_v92: bool,
) -> Result<Vec<u8>, ReadRawSaveDataError> {
    let mut result = vec![];

    let available = reader.read_u16().ok_or_eof()?;
    result.push(available.to_le_bytes()[0]);
    result.push(available.to_le_bytes()[1]);

    if !is_v92 {
        result.push(reader.read_u8().ok_or_eof()?);
    }

    for _ in 0..available.count_ones() {
        let value = reader.read_u32().ok_or_eof()?;
        result.push(value.to_le_bytes()[0]);
        result.push(value.to_le_bytes()[1]);
        result.push(value.to_le_bytes()[2]);
        result.push(value.to_le_bytes()[3]);
    }

    Ok(result)
}

pub fn calculate_checksum(checksum: &mut i32, data: &[u8]) {
    for byte in data {
        let mut byte = *byte as i32;
        if *checksum < 0 {
            byte += 1;
        }

        *checksum = byte.wrapping_add(checksum.wrapping_mul(2));
    }
}
