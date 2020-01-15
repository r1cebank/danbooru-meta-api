use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[macro_use]
extern crate log;
mod response_types;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "danbooru_meta_api=debug,actix_web=debug");
    env_logger::init();
    debug!("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::stat)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:3939")?
    .run()
    .await
}
