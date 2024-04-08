use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Lock {
    items: Mutex<HashMap<String, u16>>,
}