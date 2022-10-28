use actix_web::web;
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Row, query, FromRow, Error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Feedback{
    pub id: i64,
    pub category: String,
    pub reference: String,
    pub content: String,
    pub username: String,
    pub nickname: String,
    pub applied: i64,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Feedback {
    pub async fn new() -> Self{
        let timestamp = Utc::now();
        Self{
            id: -1,
            category: "".to_string(),
            reference: "".to_string(),
            content: "".to_string(),
            username: "".to_string(),
            nickname: "".to_string(),
            source: "".to_string(),
            applied: 0,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub async fn new_from(pool: &web::Data<SqlitePool>, category: &str,
            reference: &str, content: &str, username: &str, nickname: &str,
            applied: i64, source: &str) -> Result<Feedback, Error>{
        let timestamp = Utc::now().naive_utc();
        let sql = "INSERT INTO feedback (category, reference, content,
 username, nickname, applied, source, created_at, updated_at)
 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id, category, reference,
 content, username, nickname, source, applied, created_at, updated_at;";
        query(sql)
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(timestamp)
            .bind(timestamp)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update_from(pool: &web::Data<SqlitePool>, id: i64,
            category: &str, reference: &str, content: &str, username: &str,
            nickname: &str, applied: i64, source: &str) -> Result<Feedback, Error>{
        let updated_at = Utc::now().naive_utc();
        let sql = "UPDATE feedback SET category=?, reference=?, content=?,
              username=?, nickname=?, applied=?, source=?, updated_at=?
              WHERE id=? RETURNING id, category, reference, content, username,
              nickname, source, applied, created_at, updated_at";
        query(sql)
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(updated_at)
            .bind(id)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(&self, pool: &web::Data<SqlitePool>) -> Result<bool, Error>{
        if self.id > -1{
            query(r#"DELETE FROM feedback WHERE id = $1;"#)
                .bind(self.id)
                .execute(pool.get_ref())
                .await?;
            return Ok(true);
        }
        Ok(false)
    }
    pub async fn load(pool: &web::Data<SqlitePool>, id: i64) -> Result<Self, Error>{
        let sql = "SELECT id, category, reference, content, username, nickname,
                   source, applied, created_at, updated_at FROM feedback
                   WHERE id = $1;";
        query(sql)
            .bind(id)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn save(&mut self, pool: &web::Data<SqlitePool>) -> Result<Feedback, Error>{
        self.updated_at = Utc::now();
        let sql = if self.id > -1{
            "UPDATE feedback SET category=$1, reference=$2, content=$3,
             username=$4, nickname=$5, applied=$6, source=$7, updated_at=$8
             WHERE id=$9 RETURNING id, category, reference, content, username,
             nickname, source, applied, created_at, updated_at;"
        }else{
            "INSERT INTO feedback (category, reference, content, username,
             nickname, applied, source, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id, category, reference,
             content, username, nickname, source, applied, created_at,
             updated_at;"
        };
        let mut mquery = query(sql)
            .bind(&self.category)
            .bind(&self.reference)
            .bind(&self.content)
            .bind(&self.username)
            .bind(&self.nickname)
            .bind(&self.applied)
            .bind(&self.source)
            .bind(&self.updated_at);
        if self.id > -1{
            mquery = mquery
                .bind(&self.id)
        }
        mquery
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn create(pool: web::Data<SqlitePool>, category: &str,
            reference: &str, content: &str, username: &str,
            nickname: &str, source: &str) -> Result<Feedback, Error>{
        let applied:i64 = 0;
        let created_at = Utc::now();
        let updated_at = &created_at;
        let sql = "INSERT INTO feedback (category, reference, content,
                   username, nickname, applied, source, created_at,
                   updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                   RETURNING id, category, reference, content, username,
                   nickname, source, applied, created_at, updated_at;";
        query(sql)
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(created_at)
            .bind(updated_at)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, id: i64) -> Result<Feedback, Error>{
        let sql = "SELECT id, category, reference, content, username, nickname,
                   applied, source, created_at, updated_at FROM feedback
                   WHERE id = $1";
        query(sql)
            .bind(id)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read_all(pool: web::Data<SqlitePool>) -> Result<Vec<Feedback>, Error>{
        let sql = "SELECT id, category, reference, content, username, nickname,
                   applied, source, created_at, updated_at FROM feedback";
        query(sql)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<SqlitePool>, id: i64) -> Result<Feedback, Error>{
        let updated_at = Utc::now().naive_utc();
        let sql = "UPDATE feedback SET applied = 1, updated_at = $1
                   WHERE id = $2";
        query(sql)
            .bind(updated_at)
            .bind(id)
            .map(|row: SqliteRow| Feedback{
                id: row.get("id"),
                category: row.get("category"),
                reference: row.get("reference"),
                content: row.get("content"),
                username: row.get("username"),
                nickname: row.get("nickname"),
                source: row.get("source"),
                applied: row.get("applied"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}
