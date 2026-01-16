use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDate,
    pub completions: Vec<NaiveDate>,
}

impl Habit {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            created_at: chrono::Local::now().date_naive(),
            completions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub habits: Vec<Habit>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            habits: Vec::new(),
        }
    }
}
