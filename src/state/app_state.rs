use std::collections::HashMap;

use crate::models::processed_request::ProcessedRequest;

pub struct AppState{
    pub requests: HashMap<String,ProcessedRequest>
}

impl AppState{
    pub fn new() -> Self{
        Self { requests: HashMap::new()}
    }
}