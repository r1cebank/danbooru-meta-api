use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
#[macro_use]
extern crate log;

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "danbooru_meta_api=debug,actix_web=debug");
    env_logger::init();
    debug!("Starting server");
    HttpServer::new(|| App::new().service(index).wrap(Logger::default()))
        .bind("0.0.0.0:3939")?
        .run()
        .await
}
