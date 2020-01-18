use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct StatObj {
    pub id: i32,
    pub num_posts: Option<i32>,
    pub num_tags: Option<i32>,
    pub num_ratings: Option<i32>,
}

#[derive(Queryable)]
pub struct PostTagObj {
    pub id: i32,
    pub post_id: Option<String>,
    pub tag_id: Option<String>,
}

#[derive(Queryable)]
pub struct TagObj {
    pub id: i32,
    pub tag_id: Option<String>,
    pub name: Option<String>,
    pub category: Option<i32>,
}

#[derive(Queryable)]
pub struct PostObj {
    pub id: i32,
    pub post_id: String,
    pub md5: Option<String>,
    pub rating: Option<String>,
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
    pub post_id: String,
    pub md5: Option<String>,
    pub rating: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_ext: Option<String>,
    pub file_size: Option<i32>,
    pub source: Option<String>,
    pub pixiv_id: Option<i32>,
    // pub tags: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub result: Vec<PostResponse>,
}

#[derive(Serialize, Debug, Copy, Clone)]
pub struct StatResponse {
    pub num_posts: Option<i32>,
    pub num_tags: Option<i32>,
    pub num_ratings: Option<i32>,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RandPostParam {
    pub start: i32,
    pub end: i32,
    pub size: i32,
}
