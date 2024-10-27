use db::make_db_pool;
use dotenv::dotenv;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use http::handlers::{self, create_user, login};
mod constants;
mod db;
mod http;
mod messages;
mod workers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = make_db_pool().await;
    let database_sender = workers::spawn_database_worker(pool.clone());
    let message_worker_sender = workers::spawn_message_worker(database_sender, pool.clone());

    HttpServer::new(move || {
        let message_worker_sender = message_worker_sender.clone();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/hey", web::get().to(manual_hello))
            .route(
                "/ws",
                web::get().to(move |req, stream| {
                    handlers::ws(req, stream, message_worker_sender.clone())
                }),
            )
            .route("/user", web::post().to(create_user))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
