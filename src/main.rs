mod handlers;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use handlers::{create_task::create_task, show_tasks::show_tasks};
use to_do_list::models::NewTask;

#[get("/tasks")]
async fn show_tasks_route() -> impl Responder {
    let tasks = show_tasks();

    HttpResponse::Ok().json(tasks)
}

#[post("/tasks")]
async fn create_task_route(task: web::Json<NewTask>) -> impl Responder {
    create_task(task.title.clone());

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(show_tasks_route)
            .service(create_task_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
