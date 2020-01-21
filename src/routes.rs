use crate::models;
use crate::util;
use actix_web::{get, web, Error, HttpResponse};
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
    use crate::schema::stats::dsl::*;
    let stat_rows = stats
        .filter(id.eq(1))
        .limit(1)
        .load::<models::StatsObj>(conn)
        .map_err(|_| HttpResponse::InternalServerError())?;

    if stat_rows.len() != 0 {
        Ok(HttpResponse::Ok().json(models::StatsResponse {
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
            // Get the connection from connection pool
            let conn: &SqliteConnection = &pool.get().unwrap();
            use crate::schema::post_tags;
            use crate::schema::posts;
            let post_rows = posts::dsl::posts
                .filter(posts::dsl::id.eq_any(numbers))
                .load::<models::PostObj>(conn)
                .map_err(|_| HttpResponse::InternalServerError())?;
            let mut all_posts = Vec::new();
            for row in post_rows {
                let tag_ids = post_tags::dsl::post_tags
                    .select(post_tags::dsl::tag_id)
                    .filter(post_tags::dsl::post_id.eq(row.post_id))
                    .load::<i32>(conn)
                    .map_err(|_| HttpResponse::InternalServerError())?;
                let ext = row.file_ext.unwrap();
                let location = format!("{}/{}.{}", row.post_id % 1000, row.post_id, ext);
                all_posts.push(models::PostResponse {
                    id: row.id,
                    post_id: row.post_id,
                    md5: row.md5,
                    rating: row.rating,
                    width: row.width,
                    height: row.height,
                    file_ext: ext,
                    file_size: row.file_size,
                    source: row.source,
                    pixiv_id: row.pixiv_id,
                    location: location,
                    tags: tag_ids,
                })
            }
            let num_posts = all_posts.len() as i32;
            Ok(HttpResponse::Ok().json(models::ResultResponse {
                result: all_posts,
                count: num_posts,
            }))
        }
        Err(_) => Err(HttpResponse::BadRequest())?,
    }
}
