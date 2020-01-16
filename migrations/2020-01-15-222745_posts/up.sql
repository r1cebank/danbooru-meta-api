-- Your SQL goes here
CREATE TABLE posts (id INTEGER PRIMARY KEY UNIQUE NOT NULL, post_id TEXT UNIQUE NOT NULL, md5 TEXT, rating CHAR REFERENCES ratings (name) ON DELETE CASCADE, width INTEGER, height INTEGER, file_ext TEXT, file_size INTEGER, source TEXT, pixiv_id INTEGER)
