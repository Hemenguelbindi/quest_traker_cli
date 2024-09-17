use serde::{Deserialize, Serialize};
use chrono::Utc;


#[derive(Serialize, Deserialize, Debug)]
pub struct Quest {
    pub id: u32,
    pub task: String,
    pub description: String,
    pub prize: String,
    pub status: String,
    pub created_at: String,
    pub update_at: String,
}

impl Quest {
    pub fn new(id: u32, task: String) -> Quest {
        Quest{
            id,
            task,
            description: String::new(),
            prize: String::new(),
            status: String::new(),
            created_at: Utc::now().format("%H:%M:%S %d-%m-%Y").to_string(),
            update_at: String::new(),
        }
    }
}