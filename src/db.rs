use std::{sync::{Arc}, fs::File};
use tokio::sync::Mutex;

use serde_json::from_reader;

use crate::models::Dive;

pub type Db = Arc<Mutex<Vec<Dive>>>;

pub fn init_db() -> Db {
    let file = File::open("./data/dives.json");
    match file {
        Ok(json) => {
            let dives = from_reader(json).unwrap();
            Arc::new(Mutex::new(dives))
        },
        Err(_) => {
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}
