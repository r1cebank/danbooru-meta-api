use serde::Serialize;

use rocket_contrib::databases::diesel;

#[database("metadata_database")]
pub struct MetadataDb(diesel::SqliteConnection);

#[derive(Queryable)]
pub struct StatsObj {
    pub id: i32,
    pub num_posts: i32,
    pub num_tags: i32,
    pub num_ratings: i32,
}

#[derive(Queryable)]
pub struct PostTagObj {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable)]
pub struct TagObj {
    pub id: i32,
    pub tag_id: i32,
    pub name: Option<String>,
    pub category: Option<i32>,
}

#[derive(Queryable)]
pub struct PostObj {
    pub id: i32,
    pub post_id: i32,
    pub md5: String,
    pub rating: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_ext: Option<String>,
    pub file_size: Option<i32>,
    pub source: Option<String>,
    pub pixiv_id: Option<i32>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub post_id: i32,
    pub md5: String,
    pub rating: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_ext: String,
    pub file_size: Option<i32>,
    pub source: Option<String>,
    pub pixiv_id: Option<i32>,
    pub location: String,
    pub tags: Vec<i32>,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: Vec<PostResponse>,
    pub count: i32,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: Option<String>,
    pub category: Option<i32>,
}

#[derive(Serialize, Debug, Copy, Clone)]
pub struct StatsResponse {
    pub num_posts: i32,
    pub num_tags: i32,
    pub num_ratings: i32,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(FromForm)]
pub struct RandPostParam {
    pub start: i32,
    pub end: i32,
    pub size: i32,
}

#[derive(FromForm)]
pub struct BatchParam {
    pub batch_size: u32,
    pub batch_number: u32,
}
