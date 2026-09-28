use clap_derive::Args;
use habit_kernel_core::db::DataBase;

#[derive(Debug, Args)]
pub struct DeleteArgs {
    id: uuid::Uuid,
}

impl DeleteArgs {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

pub fn delete_habit(db: &mut DataBase, args: &DeleteArgs) {
    if let Some(habit) = db.del_by_id(*args.id()) {
        println!(
            "The habit was successfully removed. [ id: {}; name: {}; ]",
            habit.metadata().id(),
            habit.metadata().name()
        );

        return;
    }

    println!("A habit with such an ID was not found.. Error: {:?}", args);
}
