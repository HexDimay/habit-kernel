pub mod data;
pub mod metadata;

pub struct Habit {
    pub metadata: metadata::Metadata,
    data: data::Data,
}
