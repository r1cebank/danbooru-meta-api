#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
use std::collections::HashMap;

mod models;
mod routes;
mod schema;
mod util;

fn main() {
    let batch_configs = models::BatchHashMap::new(HashMap::new());
    rocket::ignite()
        .manage(batch_configs)
        .attach(models::MetadataDb::fairing())
        .mount(
            "/",
            routes![
                routes::index,
                routes::stat,
                routes::tag_by_id,
                routes::rand_posts,
                routes::get_posts,
                routes::create_batch,
                routes::get_batch
            ],
        )
        .launch();
}
