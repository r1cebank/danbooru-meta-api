use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "danbooru_meta_api=debug,actix_web=debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    debug!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::index)
            .service(routes::stat)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:3939")?
    .run()
    .await
}
