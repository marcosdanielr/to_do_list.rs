mod handlers;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use handlers::show_tasks::show_tasks;

#[get("/tasks")]
async fn show_tasks_route() -> impl Responder {
    let tasks = show_tasks();

    HttpResponse::Ok().json(tasks)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(show_tasks_route))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
