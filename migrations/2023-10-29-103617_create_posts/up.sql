-- Your SQL goes here
CREATE TABLE person (
   id INTEGER NOT NULL PRIMARY KEY,
   title TEXT NOT NULL,
   photo_path TEXT NOT NULL
);

CREATE TABLE post (
    id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL,
    photo_path TEXT NOT NULL,
    is_premium BOOLEAN NOT NULL
);