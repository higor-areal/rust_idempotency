use serde::{Serialize};

//essa struct teria a resposta da primeira request
#[derive(Serialize, Clone)]
pub struct ProcessedRequest{
    pub message: String,
}