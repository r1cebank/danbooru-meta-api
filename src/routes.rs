use crate::models;
use crate::util;
use diesel::prelude::*;
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn index() -> Json<models::MessageResponse> {
    Json(models::MessageResponse {
        message: String::from("Welcome to danbooru-meta-api"),
    })
}

#[get("/stat")]
pub fn stat(conn: models::MetadataDb) -> JsonValue {
    use crate::schema::stats::dsl::*;
    let stat_row = stats.filter(id.eq(1)).first::<models::StatsObj>(&*conn);

    match stat_row {
        Ok(row) => json!(models::StatsResponse {
            num_posts: row.num_posts,
            num_ratings: row.num_ratings,
            num_tags: row.num_tags,
        }),
        Err(_) => json!(models::ErrorResponse {
            message: String::from("Error reading database"),
        }),
    }
}

#[get("/rand_posts?<params..>")]
pub fn rand_posts(conn: models::MetadataDb, params: Form<models::RandPostParam>) -> JsonValue {
    let numbers = util::get_rand_ids(params.start, params.end, params.size);
    match numbers {
        Ok(numbers) => {
            use crate::schema::post_tags;
            use crate::schema::posts;
            let post_rows = posts::dsl::posts
                .filter(posts::dsl::id.eq_any(numbers))
                .load::<models::PostObj>(&*conn)
                .unwrap();
            let mut all_posts = Vec::new();
            for row in post_rows {
                let tag_ids = post_tags::dsl::post_tags
                    .select(post_tags::dsl::tag_id)
                    .filter(post_tags::dsl::post_id.eq(row.post_id))
                    .load::<i32>(&*conn)
                    .unwrap();
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
            json!(models::ResultResponse {
                result: all_posts,
                count: num_posts,
            })
        }
        Err(_) => json!(models::ErrorResponse {
            message: String::from("Number out of bounds"),
        }),
    }
}

#[get("/get_batch?<params..>")]
pub fn get_batch(conn: models::MetadataDb, params: Form<models::BatchParam>) -> JsonValue {
    use crate::schema::post_tags;
    use crate::schema::posts;
    let post_rows = posts::dsl::posts
        .limit(params.batch_size as i64)
        .offset(((params.batch_number - 1) * params.batch_size) as i64)
        .load::<models::PostObj>(&*conn)
        .unwrap();
    let mut all_posts = Vec::new();
    for row in post_rows {
        let tag_ids = post_tags::dsl::post_tags
            .select(post_tags::dsl::tag_id)
            .filter(post_tags::dsl::post_id.eq(row.post_id))
            .load::<i32>(&*conn)
            .unwrap();
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
    json!(models::ResultResponse {
        result: all_posts,
        count: num_posts,
    })
}

#[get("/tag/<id>")]
pub fn tag_by_id(conn: models::MetadataDb, id: i32) -> JsonValue {
    use crate::schema::tags;
    let tag_info = tags::dsl::tags
        .filter(tags::dsl::tag_id.eq(id))
        .first::<models::TagObj>(&*conn);
    match tag_info {
        Ok(tag) => json!(models::TagResponse {
            id: tag.tag_id,
            name: tag.name,
            category: tag.category,
        }),
        Err(_) => json!(models::ErrorResponse {
            message: String::from("Error reading database"),
        }),
    }
}
