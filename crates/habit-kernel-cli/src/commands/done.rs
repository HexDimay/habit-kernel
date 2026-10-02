use habit_kernel_core::db::{DataBase, traits::QueryExecutor};

pub fn done_habit(db: &mut DataBase) {
    db.current_done();
}
