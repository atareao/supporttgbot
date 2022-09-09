use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode,
                http::header::ContentType, HttpRequest,
                error::ErrorBadRequest};
use serde::Serialize;
use serde_json::Value;
use sqlx::sqlite::SqlitePool;

use crate::feedback::Feedback;

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    message: String,
}


#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Rust is the best!"))
}

#[get("/feedback")]
pub async fn get_all_feedback(req: HttpRequest, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Feedback::all(req, pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}

#[get("/status")]
pub async fn status() -> Result<HttpResponse, Error>{
    let respuesta = Respuesta{
        code: 200,
        status: "Ok".to_string(),
        message: "Up and running!".to_string(),
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&respuesta)?))
}

#[post("/hook")]
pub async fn hook(post: String) -> Result<HttpResponse, Error>{
    println!("{}", post);
    let mut content: Value = serde_json::from_str(&post).unwrap();
    if let Some(message) = content.get_mut("message"){
        if let Some(text) = message.get_mut("text"){
            println!("Texto introducido: {}", &text);
        }
    }else{
        println!("Desastre");
    }
    Ok(HttpResponse::build(StatusCode::OK).body(format!("Message recieved {}", post)))
}

