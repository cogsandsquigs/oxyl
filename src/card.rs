use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
	pub id: u64,
    pub concept: String,
    pub front: String,
    pub back: String,
    pub bucket: u64,
	pub created_at: DateTime<Utc>, // time string it was created at

}

impl Card {
	pub fn title(&self) -> String {
		self.concept.clone() + ": " + &self.front.clone()
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub age: u64,
    pub created_at: DateTime<Utc>,
}
