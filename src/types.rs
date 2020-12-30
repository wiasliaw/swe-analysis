extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InputData {
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct StageData {
    pub title: String,
    pub term_split: Vec<String>,
}

#[derive(Debug, Default)]
pub struct LabelData {
    pub term: String,
    pub tf: f32,
    pub idf: f32,
}
