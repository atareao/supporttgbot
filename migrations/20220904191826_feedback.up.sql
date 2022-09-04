-- Add up migration script here
CREATE TABLE IF NOT EXISTS feedback(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    category TEXT NOT NULL,
    content TEXT NOT NULL,
    username TEXT DEFAULT "",
    nickname TEXT DEFAULT "",
    applied BOOL DEFAULT FALSE,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

