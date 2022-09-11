use serde_json::Value;
use regex::Regex;

pub fn check_key(key: &str, message: &mut Value) -> Option<String>{
    if let Some(text) = message.get_mut("text"){
        let content = text.as_str().unwrap();
        if content == format!(r#"/{}"#, key){
            return Some("".to_string());
        }
        let patron = format!(r#"/{} "#, key);
        if content.starts_with(&patron){
            return Some(content[key.len() + 1..].trim().to_string());
        }
    }
    None
}

pub fn check_comment(key: &str, message: &mut Value) -> Option<(Option<String>, Option<String>)>{
    if let Some(text) = message.get_mut("text"){
        let content = text.as_str().unwrap();
        if content == format!(r#"/{}"#, key){
            return Some((None, Some("".to_string())));
        }
        let patron = format!(r#"/{} "#, key);
        if content.starts_with(&patron){
            let contenido = content[key.len() + 1..].trim();
            let re = Regex::new(r#"^(\d*)\s+.*"#).unwrap();
            return match re.captures(contenido) {
                Some(captures) => {
                    let referencia = captures.get(1).unwrap().as_str().to_string();
                    let texto = contenido[referencia.len() + 1..].trim().to_string();
                    Some((Some(referencia), Some(texto)))
                },
                None => Some((None, Some(contenido.to_string()))),
            };
        }
    }
    None
}

pub fn get_user(message: &mut Value) -> (String, String){
    let mut name: String = "".to_string();
    let mut nick: String = "".to_string();
    if let Some(from) = message.get_mut("from"){
        if let Some(first_name) = from.get_mut("first_name"){
            name = first_name.as_str().unwrap().to_string();
        }
        if let Some(username) = from.get_mut("username"){
            nick = username.as_str().unwrap().to_string();
        }
    }
    (name, nick)
}

pub fn get_chat_id(message: &mut Value) -> Option<i64>{
    if let Some(chat) = message.get_mut("chat"){
        if let Some(id) = chat.get_mut("id"){
            return Some(id.as_i64().unwrap());
        }
    }
    None
}
