use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DbConfigError {
    #[error("database with id {0} not found")]
    IdNotFound(Uuid),
    #[error("database with name '{0}' not found")]
    NameNotFound(String),
    #[error("database with name '{0}' already exists")]
    NameAlreadyExists(String),
}
