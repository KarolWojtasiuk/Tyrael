use crate::errors::ConvertToRawSaveDataError;
use crate::info::SaveInfo;
use crate::raw::RawSaveData;
use crate::repository::DataRepository;

#[derive(Debug)]
pub struct SaveData;

impl SaveData {
    pub fn to_raw(&self, target_version: u32) -> Result<RawSaveData, ConvertToRawSaveDataError> {
        RawSaveData::from_data(self, target_version)
    }

    pub fn from_raw(raw_data: &RawSaveData) -> Self {
        raw_data.to_data()
    }

    pub fn to_info<R: DataRepository>(&self, repository: R) -> SaveInfo<R> {
        SaveInfo::from_data(self, repository)
    }

    pub fn from_info<R: DataRepository>(info: &SaveInfo<R>) -> Self {
        info.to_data()
    }
}
