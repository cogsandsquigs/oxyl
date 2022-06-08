use chrono::{DateTime, Utc};
use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use std::fs::{self, File};
use std::io::prelude::*;
use crate::{card::Card};
use serde::{Deserialize, Serialize};

pub const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
	pub last_study_date: DateTime<Utc>,
	pub num_study_days: u64,
	pub cards: Vec<Card>,
}

pub fn read_db() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: UserData = serde_json::from_str(&db_content)?;
    Ok(parsed.cards)
}

pub fn create_db() -> Result<(), Box<dyn std::error::Error>> {
	let mut db = File::create(DB_PATH)?;
	db.write_all(
		&serde_json::to_vec(
			&UserData{
				last_study_date: Utc::now(),
				num_study_days: 0,
				cards: vec![],
			},
		)?
	)?;
	Ok(())
}


pub fn add_card(concept: String, front: String, back: String) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
	let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: UserData = serde_json::from_str(&db_content)?;
	
	let card = Card {
        id: rng.gen_range(0, 9999999),
		concept: concept,
        front: front,
		back: back,
		bucket: 0,
        created_at: Utc::now(),
    };

    parsed.cards.push(card);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed.cards)
}

pub fn add_random_card_to_db() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let rng = rand::thread_rng();
    
	add_card(
		rng.sample_iter(Alphanumeric).take(10).collect(),
		rng.sample_iter(Alphanumeric).take(10).collect(),
		rng.sample_iter(Alphanumeric).take(10).collect(),
	)
}

pub fn remove_card_at_index(index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let db_content = fs::read_to_string(DB_PATH)?;
	let mut parsed: UserData = serde_json::from_str(&db_content)?;
	parsed.cards.remove(index);
	fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(())
}
