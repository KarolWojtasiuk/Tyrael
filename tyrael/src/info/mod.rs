use crate::data::SaveData;
use crate::repository::DataRepository;

#[derive(Debug)]
pub struct SaveInfo<R: DataRepository> {
    _repository: R,
}

impl<R: DataRepository> SaveInfo<R> {
    pub fn to_data(&self) -> SaveData {
        todo!()
    }

    pub fn from_data(_data: &SaveData, _repository: R) -> Self {
        todo!()
    }
}
