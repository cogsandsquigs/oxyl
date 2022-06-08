use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use std::fs::{self, File};
use std::io::prelude::*;
use crate::{card::Card};

pub const DB_PATH: &str = "./data/db.json";

pub fn read_db() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Card> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn create_db() -> Result<(), Box<dyn std::error::Error>> {
	let mut db = File::create(DB_PATH)?;
	db.write_all(b"[]")?;
	Ok(())
}


pub fn add_card(concept: String, front: String, back: String) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
	let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<Card> = serde_json::from_str(&db_content)?;
	
	let card = Card {
        id: rng.gen_range(0, 9999999),
		concept: concept,
        front: front,
		back: back,
		bucket: 0,
        created_at: Utc::now(),
    };

    parsed.push(card);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
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
	let mut parsed: Vec<Card> = serde_json::from_str(&db_content)?;
	parsed.remove(index);
	fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(())
}
