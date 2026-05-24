use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Deserialize, Serialize, Clone)]
pub struct Payment{
    client: String,
    amount: f64
}

impl Payment{
    pub fn hash(&mut self) -> Option<String>{

        let json = match serde_json::to_string(&self) {
            Ok(t) => t,
            Err(_) => return None,
        };
        let mut hasher = Sha256::new();
        
        hasher.update(json);

        let result = hasher.finalize();

        Some(format!("{:x}", result))
    }
}
