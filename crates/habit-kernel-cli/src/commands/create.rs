use clap_derive::Args;
use habit_kernel_core::db::{DataBase, traits::QueryExecutor};

#[derive(Debug, Args)]
pub struct CreateArgs {
    name: String,
}

impl CreateArgs {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn create_habit(db: &mut DataBase, args: &CreateArgs) {
    db.add_habit(args.name());
}
