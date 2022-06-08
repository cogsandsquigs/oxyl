use chrono::prelude::*;
use rand::{distributions::Alphanumeric, prelude::*};
use std::fs::{self, File};
use std::io::prelude::*;



use crate::{card::Pet};

pub const DB_PATH: &str = "./data/db.json";

pub fn read_db() -> Result<Vec<Pet>, Box<dyn std::error::Error>> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn create_db() -> Result<(), Box<dyn std::error::Error>> {
	let mut db = File::create(DB_PATH)?;
	db.write_all(b"[]")?;
	Ok(())
}

pub fn add_random_card_to_db() -> Result<Vec<Pet>, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let db_content = fs::read_to_string(DB_PATH)?;
    let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    let range: i32 = rng.gen_range(0, 1);
    let catsdogs = match range {
        0 => "cats",
        _ => "dogs",
    };

    let random_pet = Pet {
        id: rng.gen_range(0, 9999999),
        name: rng.sample_iter(Alphanumeric).take(10).collect(),
        category: catsdogs.to_owned(),
        age: rng.gen_range(1, 15),
        created_at: Utc::now(),
    };

    parsed.push(random_pet);
    fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_card_at_index(index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let db_content = fs::read_to_string(DB_PATH)?;
	let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
	parsed.remove(index);
	fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(())
}
