use db::make_db_pool;
use dotenv::dotenv;

use actix_web::{web, App, HttpServer};
use http::handlers::{group_routes, user_routes};
mod constants;
mod db;
mod http;
mod messages;
mod workers;

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
            .app_data(web::Data::new(message_worker_sender.clone()))
            .configure(group_routes)
            .configure(user_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
