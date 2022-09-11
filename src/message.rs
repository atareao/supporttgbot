use serde_json::Value;
use regex::Regex;

pub fn command(key: &str, message: &mut Value) -> bool{
    if let Some(text) = message.get_mut("text"){
        let content = text.as_str().unwrap();
        if content.starts_with(&format!(r#"/{}"#, key)){
            return true;
        }
    }
    false
}
pub fn check_key(key: &str, message: &mut Value) -> Option<String>{
    if let Some(text) = message.get_mut("text"){
        let content = text.as_str().unwrap();
        if content.contains(&format!(r#"#{}"#, key)){
            return Some(content.trim().to_string());
        }
    }
    None
}

pub fn check_comment(key: &str, message: &mut Value) -> Option<(Option<String>, Option<String>)>{
    if let Some(text) = message.get_mut("text"){
        let content = text.as_str().unwrap();
        if content.contains(&format!(r#"#{}"#, key)){
            let patron = format!(r#"#{}\s+(\d*)"#, key);
            let re = Regex::new(&patron).unwrap();
            return match re.captures(content) {
                Some(captures) => {
                    let referencia = captures.get(1).unwrap().as_str().to_string();
                    Some((Some(referencia), Some(content.to_string())))
                },
                None => Some((None, Some(content.to_string()))),
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
