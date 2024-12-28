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
    Ok(Database { db: db })
}

impl Database {
    pub fn new(filepath: Option<&str>) -> Self {
        let db: Database;
        let res: Result<Database, Box<dyn std::error::Error>>;
        if filepath.is_some() {
            res = from_file(filepath.unwrap());
        } else {
            res = Err("No filename provided".into())
        }
        if res.is_ok() {
            db = res.unwrap();
        } else {
            db = Database { db: HashMap::new() };
        }
        db
    }

    pub fn put(&mut self, key: &DatabaseKey, value: DatabaseValue) {
        match value.len() {
            0 => {
                self.db.remove(key);
            }
            _ => {
                self.db.insert(*key, DatabaseEntry { value, age: 0 });
            }
        }
    }

    pub fn get(&self, key: &DatabaseKey) -> Option<&DatabaseValue> {
        return self.db.get(key).map(|entry| &entry.value);
    }

    pub fn keep(&mut self, key: &DatabaseKey) {
        if let Some(entry) = self.db.get_mut(key) {
            entry.age = 0;
        }
    }

    pub fn age_and_purge(&mut self) {
        println!("# entries before purge: {}", self.db.len());
        for entry in self.db.values_mut() {
            entry.age += 1;
        }
        self.db.retain(|_, v| v.age <= 7);
        println!("# entries after purge: {}", self.db.len());
    }

    pub fn save(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("saving database to {}", filepath);
        let mut file = File::create(filepath)?;
        let serialized = serde_cbor::to_vec(&self.db)?;
        file.write_all(&serialized)?;
        Ok(())
    }
}

/*
impl Default for Database {
    fn default() -> Self {
        Self { db: HashMap::new() }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        println!("STORE");
        let _ = self.save("database.cbor");
    }
}
*/
