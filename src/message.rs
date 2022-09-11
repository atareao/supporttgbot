use serde_json::{Value, json};
use regex::Regex;


pub fn check_key(key: &str, message: &mut Value) -> Option<String>{
    if let Some(text) = message.get_mut("text"){
        if text.as_str().unwrap() == format!(r#"/{}"#, key){
            return Some("".to_string());
        }
        let patron = format!(r#"^/{}\s+(.*)"#, key);
        let re = Regex::new(&patron).unwrap();
        match re.captures(text.as_str().unwrap()) {
            Some(captures) => {
                if captures.len() > 0 {
                    return Some(captures.get(1).unwrap().as_str().to_string());
                }
            },
            None => {},
        }
    }
    None
}

pub fn check_comment(key: &str, message: &mut Value) -> Option<(Option<String>, Option<String>)>{
    if let Some(text) = message.get_mut("text"){
        if text.as_str().unwrap() == format!(r#"/{}"#, key){
            return Some((None, Some("".to_string())));
        }
        let patron = format!(r#"^/{}\s+(\d*)\s+(.*)"#, key);
        let re = Regex::new(&patron).unwrap();
        match re.captures(text.as_str().unwrap()) {
            Some(captures) => {
                if captures.len() > 2 {
                    return Some((Some(captures.get(1).unwrap().as_str().to_string()),
                        Some(captures.get(2).unwrap().as_str().to_string())));
                }
            },
            None => {},
        }
        let comment = check_key(key, message);
        return Some((None, comment));
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
