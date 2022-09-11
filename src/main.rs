mod message;
mod feedback;
mod routes;

use dotenv::dotenv;
use std::env;
use std::path::Path;
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use actix_web::{App, HttpServer, web::Data};
use routes::{root, status, hook, get_all_feedback, read_one_feedback, create_feedback, update_feedback};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");

    if !sqlx::Sqlite::database_exists(&db_url).await.unwrap(){
        sqlx::Sqlite::create_database(&db_url).await.unwrap()
    }

    // Migrate the database
    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
        // Productions migrations dir
        std::env::current_exe()?.parent().unwrap().join("migrations")
    } else {
        // Development migrations dir
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir)
            .join("./migrations")
    };
    println!("{}", &migrations.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");

    Migrator::new(migrations)
        .await.unwrap()
        .run(&pool)
        .await.unwrap();

    HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(root)
            .service(status)
            .service(get_all_feedback)
            .service(read_one_feedback)
            .service(create_feedback)
            .service(update_feedback)
            .service(hook)
    })
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .run()
        .await
}
