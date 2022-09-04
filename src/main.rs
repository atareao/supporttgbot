use actix_web::{App, HttpServer};

mod routes;

use dotenv::dotenv;
use std::env;
use routes::{root, status, hook};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap();

    HttpServer::new(move ||{
        App::new()
            .service(root)
            .service(status)
            .service(hook)
    })
        .bind(format!("0.0.0.0:{}", &port))
        .unwrap()
        .run()
        .await
}
