use actix_web::{web, HttpRequest};
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
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
    pub source: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Feedback {
    pub async fn new() -> Self{
        let timestamp = Utc::now().naive_utc();
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
        let id = query("INSERT INTO feedback (category, reference, content, username, nickname, applied, source, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);")
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(timestamp)
            .bind(timestamp)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Self::load(&pool, id).await
    }

    pub async fn update_from(pool: &web::Data<SqlitePool>, id: i64,
            category: &str, reference: &str, content: &str, username: &str,
            nickname: &str, applied: i64, source: &str) -> Result<Feedback, Error>{
        let updated_at = Utc::now().naive_utc();
        query(r#"UPDATE feedback SET category=?, reference=?, content=?, username=?, nickname=?, applied=?, source=?, updated_at=? WHERE id=?"#)
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(updated_at)
            .bind(id)
            .execute(pool.get_ref())
            .await.unwrap();
        Self::load(&pool, id).await
    }

    pub async fn delete(&self, pool: &web::Data<SqlitePool>) -> Result<bool, Error>{
        if self.id > -1{
            query(r#"DELETE FROM feedback WHERE id=?"#)
                .bind(self.id)
                .execute(pool.get_ref())
                .await?;
            return Ok(true);
        }
        Ok(false)
    }
    pub async fn load(pool: &web::Data<SqlitePool>, id: i64) -> Result<Self, Error>{
        query_as!(Feedback, r#"SELECT id, category, reference, content, username, nickname, applied, source, created_at, updated_at FROM feedback WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn save(&mut self, pool: &web::Data<SqlitePool>) -> Result<bool, Error>{
        if self.id > -1{
            self.updated_at = Utc::now().naive_utc();
            query(r#"UPDATE feedback SET category=?, reference=?, content=?, username=?, nickname=?, applied=?, source=?, updated_at=? WHERE id=?"#)
                .bind(&self.category)
                .bind(&self.reference)
                .bind(&self.content)
                .bind(&self.username)
                .bind(&self.nickname)
                .bind(&self.applied)
                .bind(&self.source)
                .bind(&self.updated_at)
                .execute(pool.get_ref())
                .await?;
        }else{
            self.created_at = Utc::now().naive_utc();
            self.updated_at = self.created_at;
            self.id = query("INSERT INTO feedback (category, reference, content, username, nickname, applied, source, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);")
                .bind(&self.category)
                .bind(&self.reference)
                .bind(&self.content)
                .bind(&self.username)
                .bind(&self.nickname)
                .bind(&self.applied)
                .bind(&self.source)
                .bind(&self.created_at)
                .bind(&self.updated_at)
                .execute(pool.get_ref())
                .await?
                .last_insert_rowid();
        }
        Ok(true)
    }

    pub async fn create(pool: web::Data<SqlitePool>, category: &str,
            reference: &str, content: &str, username: &str,
            nickname: &str, source: &str) -> Result<Feedback, Error>{
        let applied:i64 = 0;
        let created_at = Utc::now().naive_utc();
        let updated_at = &created_at;
        let id = query("INSERT INTO feedback (category, reference, content, username, nickname, applied, source, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);")
            .bind(category)
            .bind(reference)
            .bind(content)
            .bind(username)
            .bind(nickname)
            .bind(applied)
            .bind(source)
            .bind(created_at)
            .bind(updated_at)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Self::read(&pool, id).await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, id: i64) -> Result<Feedback, Error>{
        let feedback = query_as!(Feedback, r#"SELECT id, category, reference, content, username, nickname, applied, source, created_at, updated_at FROM feedback WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(feedback)
    }

    pub async fn read_all(pool: web::Data<SqlitePool>) -> Result<Vec<Feedback>, Error>{
            query_as!(Feedback, r#" SELECT id, category, reference, content, username, nickname, applied, source, created_at, updated_at FROM feedback"#)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<SqlitePool>, id: i64) -> Result<Feedback, Error>{
        let updated_at = Utc::now().naive_utc();
        query(r#"UPDATE feedback SET applied=1, updated_at=? WHERE id=?"#)
            .bind(updated_at)
            .bind(id)
            .execute(pool.get_ref())
            .await;
        Self::read(&pool, id).await
    }
}
