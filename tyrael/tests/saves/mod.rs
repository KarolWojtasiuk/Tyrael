use crate::*;

pub mod normal;
pub mod starters;

pub fn all() -> Vec<TestSave> {
    let mut result = starters::saves();
    result.append(&mut normal::saves());
    result
}
