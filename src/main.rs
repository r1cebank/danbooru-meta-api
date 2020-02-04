#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate log;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket;
extern crate rand_chacha;
use std::collections::HashMap;

mod db;
mod models;
mod routes;
mod schema;
mod util;

fn main() {
    let batch_configs = models::BatchHashMap::new(HashMap::new());
    rocket::ignite()
        .manage(batch_configs)
        .attach(models::MetadataDb::fairing())
        .register(catchers![routes::not_found])
        .mount(
            "/",
            routes![
                routes::index,
                routes::stat,
                routes::tag_by_id,
                routes::rand_posts,
                routes::get_posts,
                routes::create_batch,
                routes::get_train_batch,
                routes::get_test_batch,
                routes::get_validation_batch,
                routes::get_batch_info
            ],
        )
        .launch();
}
