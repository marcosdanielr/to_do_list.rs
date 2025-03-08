use self::tasks::dsl::*;
use crate::{
    configs::database::establish_connection,
    models::task_model::{NewTask, Task},
    schema::tasks,
};
use diesel::prelude::*;

pub struct TasksService {
    connection: PgConnection,
}

impl TasksService {
    pub fn new() -> Self {
        let connection = establish_connection();
        TasksService { connection }
    }

    pub fn create(&mut self, new_task: NewTask) -> Result<Task, String> {
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .get_result(&mut self.connection)
            .map_err(|err| err.to_string())
    }

    pub fn list(&mut self) -> Result<Vec<Task>, String> {
        tasks
            .select(Task::as_select())
            .load::<Task>(&mut self.connection)
            .map_err(|err| err.to_string())
    }
}
