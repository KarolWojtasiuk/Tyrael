#![feature(iter_next_chunk)]

use crate::attribute::AttributeData;
use crate::character::CharacterData;
use crate::errors::{ReadCharacterSaveError, WriteCharacterSaveError};
use crate::item::ItemData;
use crate::location::LocationData;
use crate::mercenary::MercenaryData;
use crate::npc::NpcData;
use crate::quest::QuestData;
use crate::skill::SkillData;
use crate::waypoint::WaypointData;

pub mod attribute;
pub mod character;
pub mod errors;
pub mod item;
pub mod location;
pub mod mercenary;
pub mod npc;
pub mod quest;
pub mod skill;
pub mod waypoint;

mod reader;
mod writer;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSave {
    pub version: u32,
    pub character: CharacterData,
    pub location: LocationData,
    pub mercenary: Option<MercenaryData>,
    pub quests: QuestData,
    pub waypoints: WaypointData,
    pub npcs: NpcData,
    pub attributes: AttributeData,
    pub skills: SkillData,
    pub items: ItemData,
}

impl CharacterSave {
    pub fn read(bytes: &[u8]) -> Result<Self, ReadCharacterSaveError> {
        reader::read_character_save(&mut bytes.iter())
    }

    pub fn write(&self) -> Result<Vec<u8>, WriteCharacterSaveError> {
        writer::write_character_save(self)
    }
}
