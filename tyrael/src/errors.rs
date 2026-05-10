use derive_more::{Display, Error, From};

#[derive(Error, Debug, Display)]
pub enum ReadRawSaveDataError {
    #[display("unexpected end of file")]
    UnexpectedEOF,
    #[display("invalid file signature ({signature:#010X})")]
    InvalidSignature { signature: u32 },
    #[display("unsupported save version ({version})")]
    UnuspportedVersion { version: u32 },
    #[display("mismatched header (expected: {expected:#X}, actual: {actual:#X})")]
    MismatchedHeader { expected: u32, actual: u32 },
    #[display("corrupted data ({reason})")]
    CorruptedData { reason: &'static str },
}

#[derive(Error, Debug, Display, From)]
pub enum WriteRawSaveDataError {
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug, Display, From)]
pub enum ConvertToRawSaveDataError {
    #[display("unsupported target save version ({target_version})")]
    UnuspportedVersion { target_version: u32 },
}
