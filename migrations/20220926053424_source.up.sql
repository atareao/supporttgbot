-- Add up migration script here
ALTER TABLE feedback ADD COLUMN source TEXT NOT NULL DEFAULT "";
