-- Add up migration script here
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY autoincrement NOT NULL,
    text TEXT NOT NULL,
    done BOOLEAN default false NOT NULL
);