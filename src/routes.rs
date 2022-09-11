use actix_web::{get, post, put, delete, web, Error, HttpResponse, http::StatusCode,
                http::header::ContentType, HttpRequest,
                error::{ErrorBadRequest, ErrorNotFound}};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePool;
use regex::Regex;

use crate::{feedback::Feedback, message::{check_key, get_user, check_comment, get_chat_id}, telegram::send_message};

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
        match code{
            0 ..= 299 => Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
            _ => Ok(HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&respuesta)?)),
        }
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
    Feedback::read_all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}

#[get("/feedback/{id}")]
pub async fn read_one_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        path_id: web::Path<i64>) -> Result<HttpResponse, Error>{
    let id = path_id.into_inner();
    match Feedback::read(&pool, id).await{
        Ok(feedback) => Respuesta::new(200,serde_json::to_value(feedback).unwrap()),
        Err(_) => Respuesta::simple(400, &format!("Feedback {} not found", id)),
    }
}

#[delete("/feedback/{id}")]
pub async fn delete_one_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        path_id: web::Path<i64>) -> Result<HttpResponse, Error>{
    let id = path_id.into_inner();
    match Feedback::read(&pool, id).await{
        Ok(feedback) => {
            feedback.delete(&pool).await.unwrap();
            Respuesta::new(200,serde_json::to_value(feedback).unwrap())
        },
        Err(_) => Respuesta::simple(400, &format!("Feedback {} not found", id)),
    }
}

#[put("/feedback/{id}")]
pub async fn update_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        path_id: web::Path<i64>, post: String) -> Result<HttpResponse, Error>{
    let id = path_id.into_inner();
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
    let applied = match post_content.get_mut("applied") {
        Some(value) => value.as_i64().unwrap(),
        None => 0,
    };

    match Feedback::update_from(&pool, id, &category, &reference, &content, &username, &nickname, applied)
        .await{
            Ok(feedback) => Respuesta::new(200, serde_json::to_value(feedback).unwrap()),
            Err(_) => Respuesta::simple(400, "Bad request"),
        }
}

#[post("/feedback")]
pub async fn create_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        post: String) -> Result<HttpResponse, Error>{
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
    let applied = match post_content.get_mut("applied") {
        Some(value) => value.as_i64().unwrap(),
        None => 0,
    };

    match Feedback::new_from(&pool, &category, &reference, &content, &username, &nickname, applied)
        .await{
            Ok(feedback) => Respuesta::new(200, serde_json::to_value(feedback).unwrap()),
            Err(_) => Respuesta::simple(400, "Bad request"),
        }
}

#[get("/status")]
pub async fn status() -> Result<HttpResponse, Error>{
    Respuesta::simple(200, "Up and running")
}

#[post("/hook")]
pub async fn hook(req: HttpRequest, pool: web::Data<SqlitePool>, post: String) -> Result<HttpResponse, Error>{
    println!("{}", post);
    let mut content: Value = serde_json::from_str(&post).unwrap();
    if let Some(message) = content.get_mut("message"){
        let (name, nick) = get_user(message);
        let user = if !nick.is_empty() {format!("@{}", nick)} else {name.clone()};
        let option_chat_id = get_chat_id(message);
        match check_key("idea", message){
            Some(content) => {
                if &content != "" {
                    match Feedback::new_from(&pool, "idea", "", &content, &name, &nick, 0).await{
                        Ok(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Muchas gracias por compartir tu idea {}", user);
                                send_message(chat_id, &text).await;
                            }
                        },
                        Err(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Lo siento {}, no he podido registrar tu idea. Mira que está pasando @atareao", user);
                                send_message(chat_id, &text).await;
                            }

                        },
                    }
                }
            },
            None => {},
        }
        match check_key("pregunta", message){
            Some(content) => {
                if &content != "" {
                    match Feedback::new_from(&pool, "pregunta", "", &content, &name, &nick, 0).await{
                        Ok(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Muchas gracias por tu pregunta {}", user);
                                send_message(chat_id, &text).await;
                            }
                        },
                        Err(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Lo siento {}, no he podido registrar tu pregunta. Mira que está pasando @atareao", user);
                                send_message(chat_id, &text).await;
                            }

                        },
                    }
                }
            },
            None => {},
        }
        match check_comment("comentario", message){
            Some((refer, comment)) => {
                let referencia = match refer{
                    Some(refer) => refer,
                    None => "".to_string()
                };
                let comentario = match comment{
                    Some(comment) => comment,
                    None => "".to_string()
                };

                if &comentario != ""{
                    match Feedback::new_from(&pool, "comentario", &referencia, &comentario, &name, &nick, 0).await{
                        Ok(_) => {},
                        Err(_) => {},
                    }
                }
            },
            None => {},
        }
    }else{
        println!("Desastre");
    }
    Respuesta::new(200, json!({"content": post}))
}

