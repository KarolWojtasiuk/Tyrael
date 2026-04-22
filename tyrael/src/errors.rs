use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ReadCharacterSaveError {
    #[error("unexpected end of file")]
    UnexpectedEOF,
    #[error("invalid save signature ({_0:#010x})")]
    InvalidSignature(u32),
    #[error("unsupported save version ({_0})")]
    UnsupportedVersion(u32),
    #[error("invalid character name ({0})")]
    InvalidCharacterName(CharacterNameError),
    #[error("invalid character progression (expansion: {expansion}, progression: {progression})")]
    InvalidCharacterProgression { expansion: bool, progression: u8 },
    #[error("invalid character active weapon set ({_0})")]
    InvalidCharacterWeaponSet(u8),
    #[error("invalid character class ({_0:#010b})")]
    InvalidCharacterClass(u8),
    #[error("invalid save location ({_0:#010x})")]
    InvalidSaveLocation(u32),
    #[error("invalid mercenary dead flag ({_0})")]
    InvalidMercenaryDeadFlag(u16),
    #[error("invalid mercenary kind ({_0})")]
    InvalidMercenaryKind(u16),
    #[error("invalid magic value (expected: {expected:?}, actual: {actual:?})")]
    InvalidMagicValue { expected: Vec<u8>, actual: Vec<u8> },
}

#[derive(Error, Debug, PartialEq)]
pub enum WriteCharacterSaveError {}

#[derive(Error, Debug, PartialEq)]
pub enum CharacterNameError {
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
}
