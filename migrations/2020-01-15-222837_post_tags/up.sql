-- Your SQL goes here
CREATE TABLE post_tags (id INTEGER PRIMARY KEY AUTOINCREMENT, post_id TEXT REFERENCES posts (post_id) ON DELETE CASCADE NOT NULL, tag_id TEXT REFERENCES tags (tag_id) ON DELETE CASCADE NOT NULL)
