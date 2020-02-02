use crate::models;
use diesel::prelude::*;

pub fn get_stat(conn: models::MetadataDb) -> Result<models::StatsObj, diesel::result::Error> {
    use crate::schema::stats::dsl::*;
    stats.filter(id.eq(1)).first::<models::StatsObj>(&*conn)
}

pub fn get_posts_by_id(
    conn: models::MetadataDb,
    ids: &Vec<u32>,
) -> Result<Vec<models::PostResponse>, diesel::result::Error> {
    use crate::schema::post_tags;
    use crate::schema::posts;
    let post_rows = posts::dsl::posts
        .filter(posts::dsl::id.eq_any(ids.into_iter().map(|n| *n as i32)))
        .load::<models::PostObj>(&*conn)?;
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
    Ok(all_posts)
}
