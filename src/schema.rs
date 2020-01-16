table! {
    post_tags (id) {
        id -> Nullable<Integer>,
        post_id -> Text,
        tag_id -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        post_id -> Text,
        md5 -> Nullable<Text>,
        rating -> Nullable<Text>,
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
        num_posts -> Nullable<Integer>,
        num_tags -> Nullable<Integer>,
        num_ratings -> Nullable<Integer>,
    }
}

table! {
    tags (id) {
        id -> Integer,
        tag_id -> Nullable<Text>,
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
