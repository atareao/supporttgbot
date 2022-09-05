-- Add up migration script here
CREATE TABLE IF NOT EXISTS feedback(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    category TEXT NOT NULL,
    content TEXT NOT NULL,
    username TEXT NOT NULL DEFAULT "",
    nickname TEXT NOT NULL DEFAULT "",
    applied INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

