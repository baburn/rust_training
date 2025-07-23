use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::models::KeyValue;

pub struct KVDatabase {
    store: Vec<Box<KeyValue>>,
    file: File,
}

impl KVDatabase {
    pub fn new(filename: &str) -> Self {
        let path = Path::new(filename);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .expect("Unable to open file");

        let mut store = Vec::new();
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let kv = KeyValue::new(parts[0].to_string(), parts[1].to_string());
                    store.push(kv);
                }
            }
        }

        Self { store, file }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let kv = KeyValue::new(key.clone(), value.clone());
        writeln!(self.file, "{}={}", key, value).expect("Write failed");
        self.store.push(kv);
    }

    pub fn get_all(&self) -> &[Box<KeyValue>] {
        &self.store
    }
}

impl Drop for KVDatabase {
    fn drop(&mut self) {
        println!("💾 DB dropped. Auto-save done via RAII.");
    }
}
