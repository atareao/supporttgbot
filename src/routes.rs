use actix_web::{get, post, put, delete, web, Error, HttpResponse, http::StatusCode,
                http::header::ContentType, HttpRequest,
                error::{ErrorBadRequest, ErrorNotFound}};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::sqlite::SqlitePool;
use std::env;
use reqwest::header::AUTHORIZATION;

use crate::{feedback::Feedback, message::{check_key, get_user, check_comment,
    command, get_chat_id}, telegram::send_message};

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
pub async fn root(req: HttpRequest) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
    Respuesta::simple(200, "Rust es lo mejor!")
}

#[get("/feedback")]
pub async fn get_all_feedback(req: HttpRequest, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
    Feedback::read_all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}

#[get("/feedback/{id}")]
pub async fn read_one_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        path_id: web::Path<i64>) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
    let id = path_id.into_inner();
    match Feedback::read(&pool, id).await{
        Ok(feedback) => Respuesta::new(200,serde_json::to_value(feedback).unwrap()),
        Err(_) => Respuesta::simple(400, &format!("Feedback {} not found", id)),
    }
}

#[delete("/feedback/{id}")]
pub async fn delete_one_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        path_id: web::Path<i64>) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
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
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
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
    let source = match post_content.get_mut("source") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => "".to_string(),
    };

    match Feedback::update_from(&pool, id, &category, &reference, &content, &username, &nickname, applied, &source)
        .await{
            Ok(feedback) => Respuesta::new(200, serde_json::to_value(feedback).unwrap()),
            Err(_) => Respuesta::simple(400, "Bad request"),
        }
}

#[post("/feedback")]
pub async fn create_feedback(req: HttpRequest, pool: web::Data<SqlitePool>,
        post: String) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
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
    let source = match post_content.get_mut("source") {
        Some(value) => value.as_str().unwrap().to_string(),
        None => "".to_string(),
    };

    match Feedback::new_from(&pool, &category, &reference, &content, &username, &nickname, applied, &source)
        .await{
            Ok(feedback) => Respuesta::new(200, serde_json::to_value(feedback).unwrap()),
            Err(_) => Respuesta::simple(400, "Bad request"),
        }
}

#[get("/status")]
pub async fn status(req: HttpRequest) -> Result<HttpResponse, Error>{
    let token = format!("Bearer {}", env::var("TOKEN").expect("TOKEN not set"));
    if !req.headers().contains_key(AUTHORIZATION) || 
            req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap() != token{
        return Respuesta::simple(401, "Unauthorized");
    }
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
        if command("ayuda", message){
            let text = "Ayuda:
¿Como colaborar con tus ideas, preguntas y comentarios?

Utilizando `hastags` (#),

* Para sugerir una idea, utiliza `#idea`. Por ejemplo, `#idea esta  es una buena idea`

* En el caso de que quieras hacer una pregunta para los capítulos de preguntas y respuestas, utiliza `#pregunta`. Por ejemplo `¿Cuanto duermes? #pregunta`

* Si lo que quieres es hacer un comentario a un podcast utiliza `#comentario`. Por ejemplo `#comentario 123 me gusta`. Este comentario en concreto irá al podcast número 123

Indicarte que `#idea`, `#pregunta`, `#comentario` no tienen que ir necesariamenta al principio o al final del mensaje, pueden ir donde tu quieras.
";
            if let Some(chat_id) = option_chat_id{
                send_message(chat_id, text).await;
            }
        };
        match check_key("idea", message){
            Some(content) => {
                if &content == ""{
                    if let Some(chat_id) = option_chat_id{
                        let text = format!("Tienes que escribir `/idea` seguido del contenido, {}", user);
                        send_message(chat_id, &text).await;
                    }
                }else{
                    match Feedback::new_from(&pool, "idea", "", &content, &name, &nick, 0, "Telegram").await{
                        Ok(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Muchas gracias por compartir tu idea {}", user);
                                send_message(chat_id, &text).await;
                            }
                        },
                        Err(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Lo siento {}, no he podido registrar tu idea. Mira que está pasando @atareao!", user);
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
                if &content == ""{
                    if let Some(chat_id) = option_chat_id{
                        let text = format!("Tienes que escribir `/pregunta` seguido del contenido, {}", user);
                        send_message(chat_id, &text).await;
                    }
                } else {
                    match Feedback::new_from(&pool, "pregunta", "", &content, &name, &nick, 0, "Telegram").await{
                        Ok(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Muchas gracias por tu pregunta {}", user);
                                send_message(chat_id, &text).await;
                            }
                        },
                        Err(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Lo siento {}, no he podido registrar tu pregunta. Mira que está pasando @atareao!", user);
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
                    match Feedback::new_from(&pool, "comentario", &referencia, &comentario, &name, &nick, 0, "Telegram").await{
                        Ok(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Muchas gracias por tu comentario {}", user);
                                send_message(chat_id, &text).await;
                            }
                        },
                        Err(_) => {
                            if let Some(chat_id) = option_chat_id{
                                let text = format!("Lo siento {}, no he podido registrar tu comentario. Mira que está pasando @atareao!", user);
                                send_message(chat_id, &text).await;
                            }

                        },
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

