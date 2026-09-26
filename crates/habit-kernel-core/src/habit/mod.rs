#[macro_use]
pub mod data;
pub mod metadata;

#[derive(Debug)]
pub struct Habit {
    pub metadata: metadata::Metadata,
    data: data::Data,
}
