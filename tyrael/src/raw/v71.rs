use std::io::Write;

use bitter::{BitWriter, LittleEndianReader, LittleEndianWriter};

use crate::data::SaveData;
use crate::errors::{ReadRawSaveDataError, WriteRawSaveDataError};
use crate::raw::common::{ReaderExt, read_attributes_v71_v87_v89};
use crate::raw::{
    ATTRIBUTES_HEADER,
    ITEMS_HEADER,
    NPCS_HEADER,
    QUESTS_HEADER,
    SKILLS_HEADER,
    WAYPOINTS_HEADER,
};

#[derive(Debug)]
pub struct RawSaveDataV71 {
    pub character: [u8; 122],
    pub quests: [u8; 294],
    pub waypoints: [u8; 78],
    pub npcs: [u8; 50],
    pub attributes: Vec<u8>,
    pub skills: [u8; 30],
    pub items: Vec<u8>,
}

impl RawSaveDataV71 {
    pub fn read(mut reader: LittleEndianReader) -> Result<Self, ReadRawSaveDataError> {
        let character = reader.read_array::<122>()?;

        reader.assert_section_header_u32(QUESTS_HEADER)?;
        let quests = reader.read_array::<294>()?;

        reader.assert_section_header_u16(WAYPOINTS_HEADER)?;
        let waypoints = reader.read_array::<78>()?;

        reader.assert_section_header_u16(NPCS_HEADER)?;
        let npcs = reader.read_array::<50>()?;

        reader.assert_section_header_u16(ATTRIBUTES_HEADER)?;
        let attributes = read_attributes_v71_v87_v89(&mut reader)?;

        reader.assert_section_header_u16(SKILLS_HEADER)?;
        let skills = reader.read_array::<30>()?;

        reader.assert_section_header_u16(ITEMS_HEADER)?;
        let items = reader.remainder().data().to_vec();

        Ok(Self {
            character,
            quests,
            waypoints,
            npcs,
            attributes,
            skills,
            items,
        })
    }

    pub fn write<W: std::io::Write>(
        &self,
        mut writer: LittleEndianWriter<W>,
    ) -> Result<(), WriteRawSaveDataError> {
        writer.write_all(&self.character)?;

        writer.write_u32(QUESTS_HEADER)?;
        writer.write_all(&self.quests)?;

        writer.write_u16(WAYPOINTS_HEADER)?;
        writer.write_all(&self.waypoints)?;

        writer.write_u16(NPCS_HEADER)?;
        writer.write_all(&self.npcs)?;

        writer.write_u16(ATTRIBUTES_HEADER)?;
        writer.write_all(&self.attributes)?;

        writer.write_u16(SKILLS_HEADER)?;
        writer.write_all(&self.skills)?;

        writer.write_u16(ITEMS_HEADER)?;
        writer.write_all(&self.items)?;

        Ok(())
    }

    pub fn to_data(&self) -> SaveData {
        todo!()
    }

    pub fn from_data(_data: &SaveData) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bitter::{LittleEndianReader, LittleEndianWriter};
    use test_case::test_case;

    use crate::raw::VERSION_HEADER_SIZE;
    use crate::raw::v71::RawSaveDataV71;
    use crate::tests::TestSave;
    use crate::tests::saves::classic::*;

    #[test_case(v1_00::CLASSIC_AMAZON_STARTER)]
    #[test_case(v1_01::CLASSIC_BARBARIAN_STARTER)]
    #[test_case(v1_02::CLASSIC_NECROMANCER_STARTER)]
    #[test_case(v1_03::CLASSIC_PALADIN_STARTER)]
    #[test_case(v1_04c::CLASSIC_SORCERESS_STARTER)]
    #[test_case(v1_05b::CLASSIC_SORCERESS_STARTER)]
    #[test_case(v1_06b::CLASSIC_SORCERESS_STARTER)]
    fn data_is_read_and_written_losslessly(save: TestSave) {
        let input = &save.bytes[VERSION_HEADER_SIZE..];
        let reader = LittleEndianReader::new(input);
        let data = RawSaveDataV71::read(reader)
            .unwrap_or_else(|e| panic!("Cannot read raw save data ({e})"));

        let mut output = vec![];
        data.write(LittleEndianWriter::new(&mut output))
            .unwrap_or_else(|e| panic!("Cannot write raw save data ({e})"));

        assert_eq!(input, output);
    }
}
