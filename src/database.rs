use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use crate::message;

pub type DatabaseKey = [u8; message::KEY_LENGTH];
pub type DatabaseValue = Vec<u8>;
#[derive(serde::Deserialize, serde::Serialize)]
struct DatabaseEntry {
    value: DatabaseValue,
    age: u8,
}

pub struct Database {
    db: HashMap<DatabaseKey, DatabaseEntry>,
}

fn from_file(filepath: &str) -> Result<Database, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let db: HashMap<DatabaseKey, DatabaseEntry> = serde_cbor::from_slice(&buf)?;
    println!("loaded database from {}", filepath);
    Ok(Database { db })
}

impl Database {
    pub fn new(filepath: Option<&str>) -> Self {
        match filepath {
            Some(file) => from_file(file).unwrap_or_else(|e| {
                eprintln!("failed to load {}: {}", file, e);
                Database { db: HashMap::new() }
            }),
            None => Database { db: HashMap::new() },
        }
    }

    pub fn put(&mut self, key: &DatabaseKey, value: DatabaseValue) {
        if value.is_empty() {
            self.db.remove(key);
        } else {
            self.db.insert(*key, DatabaseEntry { value, age: 0 });
        }
    }

    pub fn get(&self, key: &DatabaseKey) -> Option<&DatabaseValue> {
        self.db.get(key).map(|entry| &entry.value)
    }

    pub fn keep(&mut self, key: &DatabaseKey) {
        if let Some(entry) = self.db.get_mut(key) {
            entry.age = 0;
        }
    }

    pub fn age_and_purge(&mut self) {
        println!("# entries before purge: {}", self.db.len());
        self.db.values_mut().for_each(|entry| entry.age += 1);
        self.db.retain(|_, v| v.age <= 7);
        println!("# entries after purge: {}", self.db.len());
    }

    pub fn save(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("saving database to {}", filepath);
        let serialized = serde_cbor::to_vec(&self.db)?;
        let mut file = File::create(filepath)?;
        file.write_all(&serialized)?;
        Ok(())
    }
}
