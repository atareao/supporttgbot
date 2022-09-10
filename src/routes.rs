use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode,
                http::header::ContentType, HttpRequest,
                error::{ErrorBadRequest, ErrorNotFound}};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePool;

use crate::feedback::Feedback;

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    content: Value,
}
impl Respuesta {
    fn new(code: i32, content: Value) -> Result<HttpResponse, Error>{
        let respuesta = Respuesta{
            code,
            status: if code < 300 {"OK".to_string()} else {"KO".to_string()},
            content,
        };
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&respuesta)?));
    }

    fn simple(code: i32, message: &str) -> Result<HttpResponse, Error>{
        Respuesta::new(code, json!({"description": message}))
    }

}


#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Respuesta::simple(200, "Rust es lo mejor!")
}

#[get("/feedback")]
pub async fn get_all_feedback(req: HttpRequest, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Feedback::all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}

#[get("/feedback/{id}")]
pub async fn read_one_feedback(req: HttpRequest, pool: web::Data<SqlitePool>, path_id: web::Path<i64>) -> Result<HttpResponse, Error>{
    let id = path_id.into_inner();
    match Feedback::get(pool, id).await{
        Ok(feedback) => Respuesta::new(200, serde_json::to_value(feedback).unwrap()),
        Err(_) => Respuesta::simple(400, &format!("Feedback {} not found", id)),
    }
}


#[post("/feedback")]
pub async fn create_feedback(req: HttpRequest, pool: web::Data<SqlitePool>, post: String) -> Result<HttpResponse, Error>{
    let mut post_content: Value = serde_json::from_str(&post).unwrap();
    let category = match post_content.get_mut("category") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, category is mandatory")
    };
    let reference = match post_content.get_mut("reference") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => "".to_string(),
    };
    let content = match post_content.get_mut("content") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => return Respuesta::simple(400, "Bad request!, content is mandatory")
    };
    let username = match post_content.get_mut("username") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => "".to_string(),
    };
    let nickname = match post_content.get_mut("nickname") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => "".to_string(),
    };
    Feedback::new(pool, &category, &reference, &content, &username, &nickname)
        .await
        .map(|feedback| HttpResponse::Ok().json(feedback))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[get("/status")]
pub async fn status() -> Result<HttpResponse, Error>{
    Respuesta::simple(200, "Up and running")
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
    Respuesta::new(200, json!({"content": post}))
}

