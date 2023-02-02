use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Dive {
    pub guid: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub dive_time_in_minutes: u32,
    pub max_depth: u32,
}