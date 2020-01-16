use serde::Serialize;

#[derive(Queryable)]
pub struct StatObj {
    pub id: i32,
    pub num_posts: Option<i32>,
    pub num_tags: Option<i32>,
    pub num_ratings: Option<i32>,
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
