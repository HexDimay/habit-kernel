use std::path::Path;

/// Трейт реализация для получения известного пути к файлам структур.
pub trait GetPath {
    fn get_path<P>() -> P where P: AsRef<Path>;
}

/// Трейт реализация для структур подверженных сохранению в fs;
pub trait Save {
    fn save(&self) -> anyhow::Result<()> where Self: serde::Serialize + GetPath;
}

/// Трейт реализация для структур подверженных загрузке из fs;
pub trait Load {
    fn load<'a, T>() -> anyhow::Result<T> where T: serde::Deserialize<'a> + GetPath;
}