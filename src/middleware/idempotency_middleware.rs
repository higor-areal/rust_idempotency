use axum::{
    body::Body, 
    extract::State, 
    http::{
        HeaderMap, Request, StatusCode}, 
    middleware::Next, 
    response::{IntoResponse, Response},
};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::app_state::AppState;

pub async fn idempotency_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    request: Request<Body>,
    next: Next,
) -> Response {

    let key = match get_idempotency_key(request.headers()) {
        Some(key) => key,
        None => return bad_request("Idempotency-Key ausente")
    };


    let data = state.lock().await;

    if let Some(res) = data.requests.get(&key) {
        return (
            StatusCode::OK,
            res.message.clone()
        ).into_response();
    }
    drop(data);


    let mut req = request;

    req.extensions_mut().insert(key);

    next.run(req).await

}



fn get_idempotency_key(header: &HeaderMap) -> Option<String> {
    header
        .get("Idempotency-Key")?
        .to_str()
        .ok()
        .map(String::from)
}

fn bad_request(msg: &str) -> Response {
    (StatusCode::BAD_REQUEST, msg.to_string()).into_response()
}