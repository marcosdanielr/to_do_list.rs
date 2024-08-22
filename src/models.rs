use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Serialize)]

pub struct Task {
    pub id: i32,
    pub title: String,
    pub is_completed: bool,
}
