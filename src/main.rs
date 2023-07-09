mod message;
mod feedback;
mod routes;
mod telegram;
mod mattermost;
mod zinc;

use dotenv::dotenv;
use std::{env, collections::HashMap};
use std::path::Path;
use sqlx::{sqlite::SqlitePoolOptions, migrate::{Migrator, MigrateDatabase}};
use actix_web::{App, HttpServer, web::Data, middleware::Logger};
use routes::{root, status, hook, get_all_feedback, read_one_feedback, create_feedback, update_feedback};
use mattermost::Mattermost;
use zinc::Zinc;
use env_logger::Env;

#[derive(Debug, Clone)]
pub struct Channels{
    idea: String,
    comentario: String,
    pregunta: String,
    mencion: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");
    let mattermost_base_uri = env::var("MATTERMOST_BASE_URI").expect("Not found Mattermost Base Uri");
    let mattermost_token = env::var("MATTERMOST_ACCESS_TOKEN").expect("Not found Mattermost token");
    let mattermost = Mattermost::new(&mattermost_base_uri, &mattermost_token);
    let zinc_base_url = env::var("ZINC_BASE_URL").expect("Not found zinc base url");
    let zinc_indice = env::var("ZINC_INDICE").expect("Not found zinc indice");
    let zinc_token = env::var("ZINC_TOKEN").expect("Not found token");
    let zinc = Zinc::new(&zinc_base_url, &zinc_indice, &zinc_token);

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


    let channels = Channels{
        idea: mattermost.get_channel_by_name("atareao_idea").await.unwrap(),
        pregunta: mattermost.get_channel_by_name("atareao_pregunta").await.unwrap(),
        comentario: mattermost.get_channel_by_name("atareao_comentario").await.unwrap(),
        mencion: mattermost.get_channel_by_name("atareao_mencion").await.unwrap(),
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move ||{
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(mattermost.clone()))
            .app_data(Data::new(channels.clone()))
            .app_data(Data::new(zinc.clone()))
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
