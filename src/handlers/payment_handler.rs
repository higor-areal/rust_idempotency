

use axum::{
    Json, extract::{Extension, State},
    http::StatusCode, response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{models::{payment::Payment, processed_request::ProcessedRequest}, responses::response::response_creating, state::app_state::AppState};
use crate::responses::response::response;
use crate::models::processed_request::{RequestStatus};


pub async fn root() -> &'static str{
    "Root API"
}

pub async fn payment(
    State(state): State<Arc<Mutex<AppState>>>,
    Extension(key): Extension<String>,
    Json(_payload): Json<Payment>
) -> Response{

    let msg = "Pagamento processado".to_string();

    //aqui nos vamos somente editar o campo de response

    let mut data = state.lock().await;

    let process = match data.requests.get_mut(&key){
        Some(t) => t,
        None => return response(StatusCode::BAD_REQUEST, "Erro ao buscar key".to_string())
    };

    process.response = Some(msg);
    process.status = RequestStatus::Completed;
    process.status_code = Some(StatusCode::CREATED.as_u16());

    response_creating(process.clone())

}

pub async fn proccess(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<ProcessedRequest>>{
    let data = state.lock().await;

    let res: Vec<ProcessedRequest> = data.requests.values().cloned().collect();

    Json(res)
}