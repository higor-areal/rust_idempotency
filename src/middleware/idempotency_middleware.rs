use axum::{
    body::{Body, to_bytes},
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    models::{
        payment::Payment, 
        processed_request::{
            ProcessedRequest,
        }
    }, 
    state::app_state::AppState,
    responses::response::response_created
};

pub async fn idempotency_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {

    let key = match get_idempotency_key(request.headers()) {
        Some(key) if valid_key(&key) => key,
        _ => return bad_request("Idempotency-Key inválida"),
    };

    //nesse trecho aqui eu só fiz o que a ia pediu, até entendo que middleware não aceita dados como json mas que coisa estranha
    // ######
    let body = std::mem::take(request.body_mut());

    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return bad_request("Erro ao ler body"),
    };

    let mut payment: Payment = match serde_json::from_slice(&bytes) {
        Ok(payment) => payment,
        Err(_) => return bad_request("JSON inválido"),
    };

    *request.body_mut() = Body::from(bytes.clone());

    // ######

    let hash = match payment.hash() {
        Some(hash) => hash,
        None => return bad_request("Erro ao gerar hash")
    };

    let mut data = state.lock().await;

    if let Some(res) = data.requests.get(&key) {
        return request_proccessed(res, hash);
    }

    let payload = match ProcessedRequest::new(hash) {
        Some(payload) => payload,
        None => return bad_request("Erro ao criar payload")
    };

    data.requests.insert(key.clone(), payload);

    drop(data);

    request.extensions_mut().insert(key);

    next.run(request).await
}



fn get_idempotency_key(header: &HeaderMap) -> Option<String> {
    header
        .get("Idempotency-Key")?
        .to_str()
        .ok()
        .map(String::from)
}

//validação simples de key
pub fn valid_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    if key.len() < 16 || key.len() > 64 {
        return false;
    }

    key.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '-' || c == '_'
    })
}

fn bad_request(msg: &str) -> Response {
    (StatusCode::BAD_REQUEST, msg.to_string()).into_response()
}

fn request_proccessed(res: &ProcessedRequest, hash: String) -> Response{

    if res.payload_hash != hash {
        return (
            StatusCode::CONFLICT,
            "Payload diferente"
        ).into_response();
    }

    //aqui eu devo achar um jeito de pendurar esse request até que o status de res.status seja diferente de None, mas por enquanto vamos deixar simples
    response_created(res.clone())
}