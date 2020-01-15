use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub struct StatObj {
    pub num_posts: i32,
    pub num_tags: i32,
    pub num_ratings: i32,
}

#[derive(Serialize)]
pub struct MessageObj {
    pub message: String,
}
