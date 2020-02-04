use crate::db;
use crate::models;
use crate::util;
use diesel::prelude::*;
use rand::{thread_rng, SeedableRng};
use rocket::http::Status;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

#[get("/")]
pub fn index() -> models::ApiResponse {
    models::ApiResponse {
        json: json!(models::MessageResponse {
            message: String::from("Welcome to danbooru-meta-api"),
        }),
        status: Status::Ok,
    }
}

#[get("/stat")]
pub fn stat(conn: models::MetadataDb) -> models::ApiResponse {
    let stat_row = db::get_stat(conn);

    match stat_row {
        Ok(row) => models::ApiResponse {
            json: json!(models::StatsResponse {
                num_posts: row.num_posts,
                num_ratings: row.num_ratings,
                num_tags: row.num_tags,
            }),
            status: Status::Ok,
        },
        Err(_) => models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Error reading database"),
            }),
            status: Status::InternalServerError,
        },
    }
}

#[post("/posts/batch", format = "json", data = "<batch_config>")]
pub fn create_batch(
    conn: models::MetadataDb,
    batches: State<models::BatchHashMap>,
    batch_config: Json<models::BatchConfig>,
) -> models::ApiResponse {
    let mut map = batches.write().expect("rwLock is poisioned");
    let mut encoder = Uuid::encode_buffer();
    let uuid = Uuid::new_v4().to_simple().encode_lower(&mut encoder);
    // Calculate batch info, get the total posts
    let stat_row = db::get_stat(conn);

    match stat_row {
        Ok(row) => {
            // Using stat and the batch config to get batch ids
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(batch_config.seed);
            let batches = util::create_batches(
                row.num_posts as u32,
                batch_config.batch_size,
                batch_config.validation_split,
                batch_config.test_split,
                &mut rng,
            );
            map.insert(uuid.to_string(), batches);
            models::ApiResponse {
                json: json!(models::BatchIdResponse {
                    id: uuid.to_string()
                }),
                status: Status::Ok,
            }
        }
        Err(_) => models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Error reading database"),
            }),
            status: Status::InternalServerError,
        },
    }
}

#[get("/posts/batch/<id>/info")]
pub fn get_batch_info(batches: State<models::BatchHashMap>, id: String) -> models::ApiResponse {
    let map = batches.read().expect("rwLock is poisioned");
    if let Some(batch) = map.get(&id) {
        models::ApiResponse {
            json: json!(models::BatchInfoResponse {
                total_batches: batch.num_batches as usize,
                train_batches: batch.train.len(),
                validation_batches: batch.validation.len(),
                test_batches: batch.test.len(),
            }),
            status: Status::Ok,
        }
    } else {
        models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Batch not found")
            }),
            status: Status::NotFound,
        }
    }
}

#[get("/posts/batch/<id>/validation/<batch_number>")]
pub fn get_validation_batch(
    conn: models::MetadataDb,
    batches: State<models::BatchHashMap>,
    id: String,
    batch_number: usize,
) -> models::ApiResponse {
    let map = batches.read().expect("rwLock is poisioned");
    if let Some(batch) = map.get(&id) {
        let ids = batch.validation.get(batch_number);
        match ids {
            Some(id) => {
                let all_posts = db::get_posts_by_id(conn, id).unwrap();
                let num_posts = all_posts.len() as i32;
                models::ApiResponse {
                    json: json!(models::ResultResponse {
                        result: all_posts,
                        count: num_posts,
                    }),
                    status: Status::Ok,
                }
            }
            _ => models::ApiResponse {
                json: json!(models::ErrorResponse {
                    message: String::from("Batch out of bounds")
                }),
                status: Status::NotFound,
            },
        }
    } else {
        models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Batch not found")
            }),
            status: Status::NotFound,
        }
    }
}

#[get("/posts/batch/<id>/test/<batch_number>")]
pub fn get_test_batch(
    conn: models::MetadataDb,
    batches: State<models::BatchHashMap>,
    id: String,
    batch_number: usize,
) -> models::ApiResponse {
    let map = batches.read().expect("rwLock is poisioned");
    if let Some(batch) = map.get(&id) {
        let ids = batch.test.get(batch_number);
        match ids {
            Some(id) => {
                let all_posts = db::get_posts_by_id(conn, id).unwrap();
                let num_posts = all_posts.len() as i32;
                models::ApiResponse {
                    json: json!(models::ResultResponse {
                        result: all_posts,
                        count: num_posts,
                    }),
                    status: Status::Ok,
                }
            }
            _ => models::ApiResponse {
                json: json!(models::ErrorResponse {
                    message: String::from("Batch out of bounds")
                }),
                status: Status::NotFound,
            },
        }
    } else {
        models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Batch not found")
            }),
            status: Status::NotFound,
        }
    }
}

#[get("/posts/batch/<id>/train/<batch_number>")]
pub fn get_train_batch(
    conn: models::MetadataDb,
    batches: State<models::BatchHashMap>,
    id: String,
    batch_number: usize,
) -> models::ApiResponse {
    let map = batches.read().expect("rwLock is poisioned");
    if let Some(batch) = map.get(&id) {
        let ids = batch.train.get(batch_number);
        match ids {
            Some(id) => {
                let all_posts = db::get_posts_by_id(conn, id).unwrap();
                let num_posts = all_posts.len() as i32;
                models::ApiResponse {
                    json: json!(models::ResultResponse {
                        result: all_posts,
                        count: num_posts,
                    }),
                    status: Status::Ok,
                }
            }
            _ => models::ApiResponse {
                json: json!(models::ErrorResponse {
                    message: String::from("Batch out of bounds")
                }),
                status: Status::NotFound,
            },
        }
    } else {
        models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Batch not found")
            }),
            status: Status::NotFound,
        }
    }
}

#[get("/posts/random?<params..>")]
pub fn rand_posts(
    conn: models::MetadataDb,
    params: Form<models::RandPostParam>,
) -> models::ApiResponse {
    let mut rng = thread_rng();
    let numbers = util::get_rand_ids(params.start, params.end, params.size, &mut rng);
    match numbers {
        Ok(numbers) => {
            let all_posts = db::get_posts_by_id(conn, &numbers).unwrap();
            let num_posts = all_posts.len() as i32;
            models::ApiResponse {
                json: json!(models::ResultResponse {
                    result: all_posts,
                    count: num_posts,
                }),
                status: Status::Ok,
            }
        }
        Err(_) => models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Number out of bounds"),
            }),
            status: Status::BadRequest,
        },
    }
}

#[get("/posts?<params..>")]
pub fn get_posts(conn: models::MetadataDb, params: Form<models::BatchParam>) -> JsonValue {
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
pub fn tag_by_id(conn: models::MetadataDb, id: i32) -> models::ApiResponse {
    use crate::schema::tags;
    let tag_info = tags::dsl::tags
        .filter(tags::dsl::tag_id.eq(id))
        .first::<models::TagObj>(&*conn);
    match tag_info {
        Ok(tag) => models::ApiResponse {
            json: json!(models::TagResponse {
                id: tag.tag_id,
                name: tag.name,
                category: tag.category,
            }),
            status: Status::Ok,
        },
        Err(_) => models::ApiResponse {
            json: json!(models::ErrorResponse {
                message: String::from("Error reading database"),
            }),
            status: Status::InternalServerError,
        },
    }
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
