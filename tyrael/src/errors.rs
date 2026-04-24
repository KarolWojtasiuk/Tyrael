use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ReadCharacterSaveError {
    #[error("unexpected end of file")]
    UnexpectedEOF,
    #[error("invalid save signature ({_0})")]
    InvalidSignature(u32),
    #[error("unsupported save version ({_0})")]
    UnsupportedVersion(u32),
    #[error("invalid character data ({0})")]
    InvalidCharacterData(#[from] CharacterDataError),
    #[error("invalid location data ({0})")]
    InvalidLocationData(#[from] LocationDataError),
    #[error("invalid mercenary data ({0})")]
    InvalidMercenaryData(#[from] MercenaryDataError),
    #[error("invalid quest data ({0})")]
    InvalidQuestData(#[from] QuestDataError),
    #[error("invalid waypoint data ({0})")]
    InvalidWaypointData(#[from] WaypointDataError),
    #[error("invalid NPC data ({0})")]
    InvalidNpcData(#[from] NpcDataError),
    #[error("invalid attribute data ({0})")]
    InvalidAttributeNpcData(#[from] AttributeDataError),
    #[error("invalid skill data ({0})")]
    InvalidSkillData(#[from] SkillDataError),
    #[error("invalid item data ({0})")]
    InvalidItemData(#[from] ItemDataError),
    #[error("invalid magic value (expected: {expected:?}, actual: {actual:?})")]
    InvalidMagicValue { expected: Vec<u8>, actual: Vec<u8> },
}

#[derive(Error, Debug, PartialEq)]
pub enum WriteCharacterSaveError {
    #[error("invalid save signature ({_0})")]
    InvalidSignature(u32),
    #[error("unsupported save version ({_0})")]
    UnsupportedVersion(u32),
    #[error("invalid character data ({0})")]
    InvalidCharacterData(#[from] CharacterDataError),
    #[error("invalid location data ({0})")]
    InvalidLocationData(#[from] LocationDataError),
    #[error("invalid mercenary data ({0})")]
    InvalidMercenaryData(#[from] MercenaryDataError),
    #[error("invalid quest data ({0})")]
    InvalidQuestData(#[from] QuestDataError),
    #[error("invalid waypoint data ({0})")]
    InvalidWaypointData(#[from] WaypointDataError),
    #[error("invalid NPC data ({0})")]
    InvalidNpcData(#[from] NpcDataError),
    #[error("invalid attribute data ({0})")]
    InvalidAttributeNpcData(#[from] AttributeDataError),
    #[error("invalid skill data ({0})")]
    InvalidSkillData(#[from] SkillDataError),
    #[error("invalid item data ({0})")]
    InvalidItemData(#[from] ItemDataError),
}

#[derive(Error, Debug, PartialEq)]
pub enum CharacterDataError {
    #[error("name is too short")]
    NameTooShort,
    #[error("name is too long")]
    NameTooLong,
    #[error("name contains invalid character ('{_0}')")]
    NameContainsInvalidCharacter(char),
    #[error("name starts with dash or underscore")]
    NameStartsWithDashOrUnderscore,
    #[error("name ends with dash or underscore")]
    NameEndsWithDashOrUnderscore,
    #[error("name contains multiple dashes or underscores")]
    NameContainsMultipleDashesOrUnderscores,
    #[error("name contains other character after null")]
    NameContainsOtherCharacterAfterNull,
    #[error("invalid progression (expansion: {expansion}, progression: {progression})")]
    InvalidProgression { expansion: bool, progression: u8 },
    #[error("invalid active weapon set ({_0})")]
    InvalidActiveWeaponSet(u32),
    #[error("invalid class ({_0})")]
    InvalidClass(u16),
    #[error("invalid status ({_0})")]
    InvalidStatus(u8),
    #[error("invalid menu level ({_0})")]
    InvalidMenuLevel(u16),
}

#[derive(Error, Debug, PartialEq)]
pub enum LocationDataError {
    #[error("invalid save location ({_0})")]
    InvalidSaveLocation(u32),
    #[error("invalid seed ({_0})")]
    InvalidSeed(u32),
}

#[derive(Error, Debug, PartialEq)]
pub enum MercenaryDataError {
    #[error("invalid dead flag ({_0})")]
    InvalidDeadFlag(u16),
    #[error("invalid kind ({_0})")]
    InvalidKind(u16),
}

#[derive(Error, Debug, PartialEq)]
pub enum QuestDataError {}

#[derive(Error, Debug, PartialEq)]
pub enum WaypointDataError {}

#[derive(Error, Debug, PartialEq)]
pub enum NpcDataError {}

#[derive(Error, Debug, PartialEq)]
pub enum AttributeDataError {
    #[error("invalid attribute (bit: {bit}, value: {value})")]
    UnknownAttribute { bit: u8, value: u32 },
}

#[derive(Error, Debug, PartialEq)]
pub enum SkillDataError {}

#[derive(Error, Debug, PartialEq)]
pub enum ItemDataError {}
