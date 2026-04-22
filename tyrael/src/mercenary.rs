use derive_more::{Constructor, Display};

#[derive(Constructor, Debug, PartialEq, Clone, Copy)]
pub struct MercenaryData {
    pub seed: u32,
    pub name_id: u16,
    pub kind: MercenaryKind,
    pub experience: u32,
    pub dead: bool,
}

#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum MercenaryKind {
    None,
    Act1,
}
