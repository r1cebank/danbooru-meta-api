table! {
    post_tags (id) {
        id -> Nullable<Integer>,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        post_id -> Integer,
        md5 -> Text,
        rating -> Text,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        file_ext -> Nullable<Text>,
        file_size -> Nullable<Integer>,
        source -> Nullable<Text>,
        pixiv_id -> Nullable<Integer>,
    }
}

table! {
    ratings (id) {
        id -> Integer,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        is_sfw -> Nullable<Bool>,
    }
}

table! {
    stat (id) {
        id -> Integer,
        num_posts -> Integer,
        num_tags -> Integer,
        num_ratings -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        tag_id -> Integer,
        name -> Nullable<Text>,
        category -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    post_tags,
    posts,
    ratings,
    stat,
    tags,
);
