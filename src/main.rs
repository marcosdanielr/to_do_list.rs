use crate::services::tasks_service::TasksService;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

mod configs;
mod controllers;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tasks_service = Arc::new(Mutex::new(TasksService::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tasks_service.clone()))
            .service(web::scope("/api").configure(controllers::tasks_controller::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
