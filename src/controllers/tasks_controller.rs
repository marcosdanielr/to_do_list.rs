use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::models::task_model::NewTask;
use crate::services::tasks_service::*;

#[post("/tasks")]
pub async fn create(
    tasks_service: web::Data<Arc<Mutex<TasksService>>>,
    new_task: web::Json<NewTask>,
) -> impl Responder {
    let mut service = tasks_service.lock().unwrap();
    match service.create(new_task.into_inner()) {
        Ok(task) => HttpResponse::Created().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/tasks")]
pub async fn list(tasks_service: web::Data<Arc<Mutex<TasksService>>>) -> impl Responder {
    let mut service = tasks_service.lock().unwrap();

    match service.list() {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(list);
}
