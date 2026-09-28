use clap_derive::Args;
use habit_kernel_core::db::DataBase;

#[derive(Debug, Args)]
pub struct SelectArgs {
    id: uuid::Uuid,
}

impl SelectArgs {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

pub fn select_habit(db: &mut DataBase, args: &SelectArgs) {
    match db.select_habit(*args.id()) {
        Ok(_) => println!("Habit selected."),
        Err(e) => println!("{:?}", e),
    }
}

#[macro_export]
macro_rules! check_current_habit {
    ($db:expr) => {
        if $db.get_current_habit().is_none() {
            println!("Couldn't choose a habit.");
            return;
        }
    };
}
