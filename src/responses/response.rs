use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub fn response(status: StatusCode, msg: String) -> Response {
    (status, msg).into_response()
}