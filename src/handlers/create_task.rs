use self::models::*;
use self::schema::*;
use diesel::prelude::*;
use to_do_list::*;

pub fn create_task(title: String) -> Task {
    let connection = &mut establish_connection();

    let new_task = NewTask { title };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(connection) // O retorno é o tipo `Task`
        .expect("Error saving new task")
}
