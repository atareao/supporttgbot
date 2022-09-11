use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Message{
    chat_id: i64,
    text: String,

}

impl Message {
    fn new(chat_id: i64, text: &str) -> Self{
        Message{
            chat_id,
            text: text.to_string(),
        }
    }
}

pub async fn send_message(chat_id: i64, text: &str) -> Option<String>{
    let token = env::var("TG_TOKEN").expect("TG_TOKEN not set");
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let message = Message::new(chat_id, text);
    let client = Client::new();
    match client.post(url)
        .body(serde_json::to_string(&message).unwrap())
        .send()
        .await{
            Ok(response) => Some(response.status().to_string()),
            Err(_) => None,
        }
}
