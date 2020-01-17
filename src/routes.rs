use crate::models;
use crate::util;
use actix_web::{get, web, Error, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(models::MessageResponse {
        message: String::from("Welcome to danbooru-meta-api"),
    }))
}

#[get("/stat")]
pub async fn stat(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    // Get the connection from connection pool
    let conn: &SqliteConnection = &pool.get().unwrap();
    use crate::schema::stat::dsl::*;
    let stat_rows = stat
        .filter(id.eq(1))
        .limit(1)
        .load::<models::StatObj>(conn)
        .map_err(|_| HttpResponse::InternalServerError())?;

    if stat_rows.len() != 0 {
        Ok(HttpResponse::Ok().json(models::StatResponse {
            num_posts: stat_rows[0].num_posts,
            num_ratings: stat_rows[0].num_ratings,
            num_tags: stat_rows[0].num_tags,
        }))
    } else {
        Err(HttpResponse::InternalServerError())?
    }
}

#[get("/rand_posts")]
pub async fn rand_posts(
    pool: web::Data<Pool>,
    params: web::Query<models::RandPostParam>,
) -> Result<HttpResponse, Error> {
    let numbers = util::get_rand_ids(params.start, params.end, params.size);
    match numbers {
        Ok(numbers) => {
            println!("{:?}", numbers);
            println!("{:?}", params);
            Ok(HttpResponse::Ok().json(models::MessageResponse {
                message: String::from("Welcome to danbooru-meta-api"),
            }))
        }
        Err(_) => Err(HttpResponse::BadRequest())?,
    }
}
