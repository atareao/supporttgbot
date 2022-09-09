use actix_web::web;
use sqlx::{sqlite::{SqlitePool, SqliteQueryResult}, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Feedback{
    pub id: i64,
    pub category: String,
    pub reference: String,
    pub content: String,
    pub username: String,
    pub nickname: String,
    pub applied: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Feedback {
    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Feedback, Error>{
        let feedback = query_as!(Feedback, r#"SELECT id, category, reference, content, username, nickname, applied, created_at, updated_at FROM feedback WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(feedback)
    }
    pub async fn new(pool: web::Data<SqlitePool>, category: &str, reference: &str, content: &str,
            username: &str, nickname: &str) -> Result<Feedback, Error>{
        let applied:i64 = 0;
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        let id = query("INSERT INTO feedback (category, content, username, nickname, applied, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?);")
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(created_at)
            .bind(updated_at)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Self::get(pool, id).await
    }
}
