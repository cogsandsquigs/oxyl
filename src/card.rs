use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Card {
    created_at: String, // time string it was created at
    concept: String,
    front: String,
    back: String,
    bucket: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}
