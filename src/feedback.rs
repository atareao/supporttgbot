use sqlx::{sqlite::{SqlitePool, SqliteQueryResult}, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Feedback{
    pub id: i64,
    pub category: String,
    pub content: String,
    pub username: String,
    pub nickname: String,
    pub applied: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Feedback {
    pub async fn new(pool: web::Data<SqlitePool>, category: &str, content: &str,
            username: &str, nickname: &str) -> Result<Feedback, Error>{
        let applied = false;
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
    }
}
