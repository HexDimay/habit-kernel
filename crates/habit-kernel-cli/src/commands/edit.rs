use clap_derive::Args;
use habit_kernel_core::db::DataBase;

use crate::check_current_habit;

#[derive(Debug, Args)]
pub struct EditArgs {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    limitation: Option<habit_kernel_core::habit::metadata::LimitationValue>,
}

impl EditArgs {}

pub fn edit_habit(db: &mut DataBase, args: &EditArgs) {
    check_current_habit!(db);

    if let Some(name) = &args.name {
        db.get_mut_current_habit()
            .unwrap()
            .mut_metadata()
            .set_name(name);
    }

    if let Some(limitation) = &args.limitation {
        db.get_mut_current_habit()
            .unwrap()
            .mut_metadata()
            .set_limitation_value(*limitation);
    }
}
