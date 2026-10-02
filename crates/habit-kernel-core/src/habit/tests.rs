use chrono::NaiveDate;

use crate::{
    habit::{
        Habit,
        data::{Data, Value},
        metadata::LimitationValue,
        traits::{DataContainer, HabitEntity, Limitation, ValueEntity},
    },
    value_list,
};

#[test]
fn test_create_list() {
    let data = value_list!(@DMY:
        (1,1,2026) <= 1,
        (2,1,2026) <= 1,
        (6,1,2026) <= 1,
        (8,1,2026) <= 7
    );

    assert_eq!(
        data.get(NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
            .unwrap()
            .0,
        1
    );
    assert_eq!(
        data.get(NaiveDate::from_ymd_opt(2026, 1, 8).unwrap())
            .unwrap()
            .0,
        7
    );
}

#[test]
fn test_add_value() {
    let data = value_list!(@DMY:
        (1,1,2026) <= 1,
        (2,1,2026) <= 1,
        (3,1,2026) <= 1,
        (3,1,2026) <= 1
    );

    assert_eq!(
        data.get(NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
            .unwrap()
            .0,
        1
    );

    assert_eq!(
        data.get(NaiveDate::from_ymd_opt(2026, 1, 3).unwrap())
            .unwrap()
            .0,
        2
    );
}

#[test]
fn test_del_value() {
    let mut data = value_list!(@DMY:
        (1,1,2026) <= 1,
        (2,1,2026) <= 1,
        (3,1,2026) <= 1,
        (3,1,2026) <= 1
    );

    data.zeroing_by_date(NaiveDate::from_ymd_opt(2026, 1, 3).unwrap());

    assert_eq!(
        data.done
            .get(&NaiveDate::from_ymd_opt(2026, 1, 2).unwrap())
            .unwrap()
            .0,
        1
    );

    assert_eq!(
        data.done
            .get(&NaiveDate::from_ymd_opt(2026, 1, 3).unwrap())
            .unwrap()
            .0,
        0
    );
}

#[macro_export]
macro_rules! value_list {
(@DMY: $(($day:expr,$month:expr,$year:expr) <= $value:expr),*) => {
    {
        let mut data = Data::new();

        $(
            data.add(
                chrono::NaiveDate::from_ymd_opt($year, $month, $day).unwrap(),
                Value::new($value)
            );
        )*

        data
    }
};

(@YMD: $(($year:expr,$month:expr,$day:expr) <= $value:expr),*) => {
    {
        let mut data = Data::new();

        $(
            data.add(
                chrono::NaiveDate::from_ymd_opt($year, $month, $day).unwrap(),
                Value::new($value)
            );
        )*

        data
    }
};
}

// ---------------------------------------------------------------------
// Проверки trait-impl'ов. Вызываем через UFCS, чтобы обратиться именно к
// трейтовой реализации, а не к одноимённому inherent-методу.
// -----------------------------------------------------------------

#[test]
fn test_value_entity_roundtrip() {
    let mut value = Value::new(3);
    assert_eq!(<Value as ValueEntity<usize>>::get(&value), 3);

    <Value as ValueEntity<usize>>::set(&mut value, 9);
    assert_eq!(<Value as ValueEntity<usize>>::get(&value), 9);

    <Value as ValueEntity<usize>>::zeroing(&mut value);
    assert_eq!(<Value as ValueEntity<usize>>::get(&value), 0);
}

#[test]
fn test_data_container_add_and_count() {
    let today = chrono::Local::now().date_naive();
    let mut data = Data::new();

    <Data as DataContainer<Value, usize>>::add(&mut data, today, Value::new(4));
    <Data as DataContainer<Value, usize>>::add(&mut data, today, Value::new(1));

    assert_eq!(
        <Data as DataContainer<Value, usize>>::count_current_done(&data),
        5
    );
    assert_eq!(
        <Data as DataContainer<Value, usize>>::get(&data, today)
            .unwrap()
            .get(),
        5
    );
}

#[test]
fn test_limitation_value() {
    let mut limitation = LimitationValue::Unlimited;
    // `LimitationValue: Ord`, поэтому `min`/`max` из `Limitation` вызываем
    // через UFCS: иначе резолвится `Ord::min`/`Ord::max`.
    assert!(!limitation.is_lim());
    assert_eq!(<LimitationValue as Limitation<usize>>::min(&limitation), 0);
    assert_eq!(
        <LimitationValue as Limitation<usize>>::max(&limitation),
        usize::MAX
    );

    limitation.set_max(7);
    assert!(limitation.is_lim());
    assert_eq!(<LimitationValue as Limitation<usize>>::max(&limitation), 7);

    // Единственное представимое значение нижней границы — 0.
    limitation.set_mut(0);
    assert_eq!(<LimitationValue as Limitation<usize>>::min(&limitation), 0);
}

#[test]
fn test_habit_entity_delegates() {
    let mut habit = Habit::new();

    <Habit as HabitEntity<usize, Value, Data>>::set_name(&mut habit, "read");
    assert_eq!(
        <Habit as HabitEntity<usize, Value, Data>>::name(&habit),
        "read"
    );

    <Habit as HabitEntity<usize, Value, Data>>::set_limitation_value(
        &mut habit,
        LimitationValue::Max(2),
    );
    assert_eq!(habit.metadata().limitation_value(), LimitationValue::Max(2));

    // 1 и 2 — в пределах лимита.
    <Habit as HabitEntity<usize, Value, Data>>::increment(&mut habit);
    <Habit as HabitEntity<usize, Value, Data>>::increment(&mut habit);
    assert_eq!(
        <Habit as HabitEntity<usize, Value, Data>>::data(&habit).count_current_done(),
        2
    );

    // 3-й инкремент превышает Max(2) и обнуляет счётчик за день.
    <Habit as HabitEntity<usize, Value, Data>>::increment(&mut habit);
    assert_eq!(
        <Habit as HabitEntity<usize, Value, Data>>::data(&habit).count_current_done(),
        0
    );
}
