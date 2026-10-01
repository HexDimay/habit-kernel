pub mod atomic;

use std::path::Path;

/// Трейт реализация для получения известного пути к файлам структур.
pub trait GetPath<P>
where
    P: AsRef<Path>,
{
    fn get_path() -> P;
}

/// Трейт реализация для структур подверженных сохранению в fs;
pub trait Save<P>
where
    P: AsRef<Path>,
{
    fn save(&self) -> anyhow::Result<()>
    where
        Self: serde::Serialize + GetPath<P>;
}

/// Трейт реализация для структур подверженных загрузке из fs;
pub trait Load<P>
where
    P: AsRef<Path>,
{
    fn load<T>() -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned + GetPath<P>;
}
