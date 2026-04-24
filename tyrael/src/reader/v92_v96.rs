use bitter::LittleEndianReader;

use crate::CharacterSave;
use crate::attribute::AttributeData;
use crate::character::CharacterData;
use crate::errors::ReadCharacterSaveError;
use crate::item::ItemData;
use crate::location::LocationData;
use crate::mercenary::MercenaryData;
use crate::npc::NpcData;
use crate::quest::QuestData;
use crate::reader::common::*;
use crate::skill::SkillData;
use crate::waypoint::WaypointData;

pub fn read_character_save(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<CharacterSave, ReadCharacterSaveError> {
    let _filesize = reader.u32()?;
    let _checksum = reader.u32()?;

    Ok(CharacterSave {
        version,
        character: read_character_data(reader)?,
        location: read_location_data(reader)?,
        mercenary: Some(read_mercenary_data(reader)?),
        quests: read_quest_data(reader)?,
        waypoints: read_waypoint_data(reader)?,
        npcs: read_npc_data(reader)?,
        attributes: read_attribute_data(reader, version)?,
        skills: read_skill_data(reader)?,
        items: read_item_data(reader)?,
    })
}

fn read_character_data(
    reader: &mut LittleEndianReader,
) -> Result<CharacterData, ReadCharacterSaveError> {
    let active_weapon_set = character::read_active_weapon_set(reader.u32()?)?;
    let name = character::read_name(reader.bytes::<16>()?)?;
    let status = character::read_status(reader.u8()?)?;
    let progression = character::read_progression(reader.u8()?, status.expansion)?;

    magic::assert_magic_value_u16(0x0000, reader.u16()?)?;
    let class = character::read_class(reader.u8()? as u16)?;

    magic::assert_magic_value_u16(0x1E10, reader.u16()?)?;
    let menu_level = character::read_menu_level(reader.u8()? as u16)?;

    magic::assert_magic_value_u32(0x00000000, reader.u32()?)?;
    let last_played_at = Some(reader.u32()?);

    magic::assert_magic_value_u32(0xFFFFFFFF, reader.u32()?)?;
    let skill_shortcuts = character::read_skill_shortcuts_new(reader.bytes::<80>()?)?;
    let menu_appearance = character::read_menu_appearance(reader.bytes::<32>()?)?;

    Ok(CharacterData {
        name,
        class,
        status,
        progression,
        active_weapon_set,
        menu_level,
        menu_appearance,
        skill_shortcuts,
        last_played_at,
    })
}

fn read_location_data(
    reader: &mut LittleEndianReader,
) -> Result<LocationData, ReadCharacterSaveError> {
    let save_location = location::read_save_location_new(reader.bytes::<3>()?)?;
    let seed = reader.u32()?;

    Ok(LocationData::new(seed, save_location))
}

fn read_mercenary_data(
    reader: &mut LittleEndianReader,
) -> Result<MercenaryData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x0000, reader.u16()?)?;
    let dead = mercenary::read_mercenary_dead(reader.u16()?)?;
    let seed = reader.u32()?;
    let name_id = reader.u16()?;
    let kind = mercenary::read_mercenary_kind(reader.u16()?)?;
    let experience = reader.u32()?;

    magic::assert_magic_value([0x00; 144], reader.bytes::<144>()?)?;

    Ok(MercenaryData {
        seed,
        name_id,
        kind,
        experience,
        dead,
    })
}

fn read_quest_data(reader: &mut LittleEndianReader) -> Result<QuestData, ReadCharacterSaveError> {
    magic::assert_magic_value_u32(0x216F6F57, reader.u32()?)?;
    magic::assert_magic_value_u32(0x00000006, reader.u32()?)?;

    // TODO
    reader.bytes::<290>()?;
    Ok(QuestData)
}

fn read_waypoint_data(
    reader: &mut LittleEndianReader,
) -> Result<WaypointData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x5357, reader.u16()?)?;
    magic::assert_magic_value_u32(0x00000001, reader.u32()?)?;

    // TODO
    reader.bytes::<74>()?;
    Ok(WaypointData)
}

fn read_npc_data(reader: &mut LittleEndianReader) -> Result<NpcData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x7701, reader.u16()?)?;

    // TODO
    reader.bytes::<50>()?;
    Ok(NpcData)
}

fn read_attribute_data(
    reader: &mut LittleEndianReader,
    version: u32,
) -> Result<AttributeData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6667, reader.u16()?)?;
    match version {
        92 => attribute::read_attributes_old(reader, 92),
        96 => attribute::read_attributes_new(reader),
        _ => unreachable!("unexpected version '{}' for v92_v96 reader", version),
    }
}

fn read_skill_data(reader: &mut LittleEndianReader) -> Result<SkillData, ReadCharacterSaveError> {
    magic::assert_magic_value_u16(0x6669, reader.u16()?)?;

    // TODO
    Ok(SkillData)
}

fn read_item_data(_reader: &mut LittleEndianReader) -> Result<ItemData, ReadCharacterSaveError> {
    // TODO
    Ok(ItemData)
}
