use chrono::NaiveDate;

use crate::{
    habit::data::{Data, Value},
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
