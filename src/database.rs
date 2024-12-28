use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use crate::message;

pub type DatabaseKey = [u8; message::KEY_LENGTH];
pub type DatabaseValue = Vec<u8>;

pub struct Database {
    db: HashMap<DatabaseKey, DatabaseValue>,
}

fn from_file(filepath: &str) -> Result<Database, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let db: HashMap<DatabaseKey, DatabaseValue> = serde_cbor::from_slice(&buf)?;
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
                self.db.insert(*key, value);
            }
        }
    }

    pub fn get(&self, key: &DatabaseKey) -> Option<&DatabaseValue> {
        return self.db.get(key);
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
