use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Card {
    pub created_at: String, // time string it was created at
    pub concept: String,
    pub front: String,
    pub back: String,
    pub bucket: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub age: u64,
    pub created_at: DateTime<Utc>,
}
