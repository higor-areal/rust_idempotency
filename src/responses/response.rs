use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::models::processed_request::ProcessedRequest;

pub fn response(status: StatusCode, msg: String) -> Response {
    (status, msg).into_response()
}

pub fn response_created(payload: ProcessedRequest) -> Response {
    (StatusCode::OK, payload).into_response()
}

pub fn response_creating(payload: ProcessedRequest) -> Response {
    (StatusCode::CREATED, payload).into_response()
}