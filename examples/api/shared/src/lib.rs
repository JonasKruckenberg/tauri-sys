use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply<'a> {
    pub data: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody<'a> {
    pub id: i32,
    pub name: &'a str,
}