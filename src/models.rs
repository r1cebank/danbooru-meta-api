use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

use rocket_contrib::databases::diesel;

#[derive(Deserialize)]
pub struct BatchConfig {
    pub batch_size: u32,
    pub validation_split: u8,
    pub test_split: u8,
}

pub struct Batches {
    pub num_batches: u32,
    pub train: Vec<Vec<u32>>,
    pub validation: Vec<Vec<u32>>,
    pub test: Vec<Vec<u32>>,
}

pub type BatchHashMap = RwLock<HashMap<String, Batches>>;

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
pub struct BatchInfoResponse {
    pub total_batches: usize,
    pub train_batches: usize,
    pub validation_batches: usize,
    pub test_batches: usize,
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
pub struct BatchIdResponse {
    pub id: String,
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
    pub start: u32,
    pub end: u32,
    pub size: u32,
}

#[derive(FromForm)]
pub struct BatchParam {
    pub batch_size: u32,
    pub batch_number: u32,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
