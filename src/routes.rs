use crate::response_types;
use actix_web::{get, web, Responder};
use rusqlite::{params, Connection, OpenFlags, NO_PARAMS};

#[get("/")]
pub async fn index() -> impl Responder {
    web::Json(response_types::MessageObj {
        message: String::from("Welcome to danbooru-meta-api"),
    })
}

#[get("/stat")]
pub async fn stat() -> impl Responder {
    let conn = Connection::open_with_flags(
        "/home/siyuan/Documents/danbooru_meta/danbooru.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT num_posts, num_tags, num_ratings FROM stat")
        .unwrap();
    let stat_rows = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(response_types::StatObj {
                num_posts: row.get(0).unwrap(),
                num_tags: row.get(1).unwrap(),
                num_ratings: row.get(2).unwrap(),
            })
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<response_types::StatObj>>();
    web::Json(stat_rows[0])
}
