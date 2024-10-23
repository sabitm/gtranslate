#![allow(dead_code)]
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransResponse {
    pub sentences: Vec<Sentence>,
    pub dict: Option<Vec<Dict>>,
    pub src: String,
}

#[derive(Debug, Deserialize)]
pub struct Dict {
    pub pos: String,
    pub entry: Vec<Entry>,
    pub base_form: String,
    pub pos_enum: i64,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub word: String,
    pub reverse_translation: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sentence {
    pub trans: String,
    pub orig: String,
    pub backend: i64,
}
