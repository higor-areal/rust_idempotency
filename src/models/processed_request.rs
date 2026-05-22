use serde::{Serialize};

//essa struct teria a resposta da primeira request
#[derive(Serialize)]
pub struct ProcessedRequest{
    message: String,
}