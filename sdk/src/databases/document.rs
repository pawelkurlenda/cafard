use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Document {
    id: u32,
    data: Value,  // todo: Using serde_json::Value to store arbitrary JSON data
}