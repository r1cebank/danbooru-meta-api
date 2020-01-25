#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod schema;
mod util;

fn main() {
    rocket::ignite()
        .attach(models::MetadataDb::fairing())
        .mount(
            "/",
            routes![
                routes::index,
                routes::stat,
                routes::tag_by_id,
                routes::rand_posts
            ],
        )
        .launch();
}
