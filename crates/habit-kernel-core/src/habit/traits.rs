use std::ops;

use chrono::NaiveDate;

pub trait HabitEntity<N, V: ValueEntity<N>, D: DataContainer<V, N>>
where
    N: ops::Add + ops::Sub + ops::Mul + ops::Div,
{
    fn id(&self) -> uuid::Uuid;
    fn name(&self) -> &str;
    fn set_name(&mut self, new_name: &str);
    fn created(&self) -> &chrono::NaiveDate;
    fn limitation_value(&self) -> impl Limitation<N>;
    fn set_limitation_value(&mut self, new_limit: impl Limitation<N>);
    fn current_time(&self) -> chrono::NaiveDate;
    fn update_current_time(&mut self);
    fn increment(&mut self);
    fn increment_by_date(&mut self, date: NaiveDate);
    fn decrement(&mut self);
    fn decrement_by_date(&mut self, date: NaiveDate);
    fn data(&self) -> &D;
    fn mut_data(&mut self) -> &mut D;
}

pub trait DataContainer<V: ValueEntity<N>, N>
where
    N: ops::Add + ops::Sub + ops::Mul + ops::Div,
{
    fn get(&self, date: NaiveDate) -> Option<&V>;
    fn get_mut(&mut self, date: NaiveDate) -> Option<&mut V>;
    fn add_now(&mut self, value: V);
    fn add(&mut self, date: NaiveDate, value: V);
    fn decrement(&mut self, date: NaiveDate);
    fn zeroing_by_date(&mut self, date: NaiveDate);
    fn count_current_done(&self) -> usize;
}

pub trait ValueEntity<T>
where
    T: ops::Add + ops::Sub + ops::Mul + ops::Div,
{
    fn get(&self) -> T;
    fn set(&mut self, new_v: T);
    fn zeroing(&mut self);
}

pub trait Limitation<T>
where
    T: ops::Add + ops::Sub + ops::Mul + ops::Div,
{
    fn min(&self) -> T;
    fn set_mut(&mut self, new_min: T);
    fn max(&self) -> T;
    fn set_max(&mut self, new_max: T);
    fn is_lim(&self) -> bool;
}
