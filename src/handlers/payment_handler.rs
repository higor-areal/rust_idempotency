

use axum::{
    Json, extract::{Extension, State},
    http::StatusCode, response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{models::payment::Payment, state::app_state::AppState};
use crate::responses::response::response;
use crate::models::processed_request::ProcessedRequest;


pub async fn root() -> &'static str{
    "Root API"
}

pub async fn payment(
    State(state): State<Arc<Mutex<AppState>>>,
    Extension(key): Extension<String>,
    Json(_payload): Json<Payment>
) -> Response{

    let msg = "Pagamento processado".to_string();

    let processed_request = ProcessedRequest{
        message: msg.clone()
    };
    
    let mut data = state.lock().await;

    data.requests.insert(key.clone(), processed_request);

    response(StatusCode::CREATED, msg)

}