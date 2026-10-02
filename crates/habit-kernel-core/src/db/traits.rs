//! Данные трейты описывают работу с примитивной БД на основе JSON.
//!

use std::path::Path;

use crate::{habit::Habit, io::{Load, Save}};

/// Описание геттеров без мутабельности.
pub trait DataBaseMetadata {
    fn get_current_habit(&self) -> Option<&Habit>;
    fn get_by_idx(&self, index: usize) -> Option<&Habit>;
    fn get_by_id(&self, id: uuid::Uuid) -> Option<&Habit>;
    fn iter(&self) -> std::slice::Iter<'_, Habit>;
}

/// Получение геттеров с мутабельностью.
pub trait DataBaseMetadataMut {
    fn get_mut_current_habit(&mut self) -> Option<&mut Habit>;
    fn get_mut_by_idx(&mut self, index: usize) -> Option<&mut Habit>;
    fn get_mut_by_id(&mut self, id: uuid::Uuid) -> Option<&mut Habit>;
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Habit>;
}

/// Отправка запросов на полноценное изменение базы данных.
pub trait QueryExecutor<P>: Save<P> + Load<P>
where
    P: AsRef<Path>,
{
    fn add_habit(&mut self, name: &str);
    fn select_habit(&mut self, id: uuid::Uuid);
    fn current_done(&mut self);
    fn del_by_id(&mut self, id: uuid::Uuid);
}
