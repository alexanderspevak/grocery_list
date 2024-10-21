use db::make_db_pool;
use dotenv::dotenv;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use http::handlers::{self, create_user, login};
// create_user
mod db;
mod http;
mod messages;
mod state;

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

    let state_sender = state::spawn_state();
    HttpServer::new(move || {
        let state_sender = state_sender.clone();
        App::new()
            .app_data(pool.clone())
            .route("/hey", web::get().to(manual_hello))
            .route(
                "/ws",
                web::get().to(move |req, stream| handlers::ws(req, stream, state_sender.clone())),
            )
            .route("/user", web::post().to(create_user))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
