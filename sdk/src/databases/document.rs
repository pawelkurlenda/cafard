use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub data: Value,  // todo: Using serde_json::Value to store arbitrary JSON data
}