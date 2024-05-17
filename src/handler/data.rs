use log::error;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    // sync::atomic::{AtomicI32, Ordering::Relaxed},
};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Message {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<Message> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&file);
    match json_data {
        Ok(mut json_data) => {
            json_data.sort_by(|a, b| b.posted.cmp(&a.posted));
            json_data
        }
        Err(_e) => {
            error!("まだ投稿はありません");
            Vec::new()
        }
    }
}

pub fn get(id: i32) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&file);
    let mut message = Message::default();
    match json_data {
        Ok(json_data) => {
            if let Some(index) = json_data.iter().position(|item| item.id == id) {
                message = json_data[index].clone();
            }
        }
        Err(e) => {
            error!("{e}");
        }
    };
    message
}

// static POST_COUNT: AtomicI32 = AtomicI32::new(1);

pub fn create(mut message: Message) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&file);
    let mut messages: Vec<Message> = Vec::new();
    // message.id = POST_COUNT.fetch_add(1, Relaxed);
    match json_data {
        Ok(json_content) => {
            message.id = match json_content.iter().map(|post| post.id).max() {
                Some(max) => max + 1,
                None => 1,
            };
            messages = json_content;
        }
        Err(_e) => {
            message.id = 1;
        }
    };
    messages.push(message);
    let json_str = serde_json::to_string(&messages).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
    messages.pop().unwrap()
}

pub fn update(message: &Message) {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut messages: Vec<Message> = serde_json::from_str(&file).unwrap();
    if let Some(index) = messages.iter_mut().position(|ms| ms.id == message.id) {
        messages[index] = message.clone();
        let json_str = serde_json::to_string(&messages).unwrap();
        let _ = fs::write(DATA_FILENAME, json_str);
    }
}

pub fn delete(id: i32) {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut messages: Vec<Message> = serde_json::from_str(&file).unwrap();
    messages.retain(|message| message.id != id);
    let json_str = serde_json::to_string(&messages).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
}
