use self::models::*;
use diesel::prelude::*;
use to_do_list::*;

pub fn show_tasks() -> Result<Vec<Task>, String> {
    use self::schema::tasks::dsl::*;

    let connection = &mut establish_connection();
    match tasks.select(Task::as_select()).load::<Task>(connection) {
        Ok(results) => Ok(results),
        Err(err) => Err(err.to_string()),
    }
}
