use serde::{Serialize};
use axum::{
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};

#[derive(Serialize, Clone)]
pub enum RequestStatus {
    Processing,
    Completed,
}

//essa struct teria a resposta da primeira request
#[derive(Serialize, Clone)]
pub struct ProcessedRequest {
    pub payload_hash: String,
    pub response: Option<String>,
    pub status_code: Option<u16>,
    pub status: RequestStatus,
}

impl ProcessedRequest{
    pub fn new(hash: String) -> Option<Self>{
        
        Some(
            ProcessedRequest {
                payload_hash: hash, 
                response: None, 
                status_code: None, 
                status: RequestStatus::Processing 
            }
        )
    }
}

impl IntoResponse for ProcessedRequest {
    fn into_response(self) -> Response {
        let status = match self.status {
            RequestStatus::Processing => StatusCode::ACCEPTED,
            RequestStatus::Completed => StatusCode::OK,
        };

        (status, Json(self)).into_response()
    }
}