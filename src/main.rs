mod feedback;
mod routes;

use dotenv::dotenv;
use std::env;
use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{App, HttpServer, web::Data};
use routes::{root, status, hook};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let port = env::var("PORT").unwrap();
    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");

    HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(root)
            .service(status)
            .service(hook)
    })
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .run()
        .await
}
