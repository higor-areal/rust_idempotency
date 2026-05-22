mod handlers;
mod responses;
mod state; 
mod middleware;
mod models;

use axum::{
    Router, 
    middleware::from_fn_with_state,
    routing::{get, post}
};
use tokio::sync::Mutex;
use std::sync::Arc;

use handlers::payment_handler::{
    root,
    payment
};
use middleware::idempotency_middleware::idempotency_middleware;

use state::app_state::AppState;

#[tokio::main]
async fn main() {

    let state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
    .route("/payment", post(payment))
    .with_state(state.clone())
    .route_layer(from_fn_with_state(state, idempotency_middleware))
    .route("/", get(root))
    ;
    

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await{
        Ok(tcp)  => tcp,
        Err(_msg) => return println!("Erro no listener")
    };

    println!("Start");

    let _server = match axum::serve(listener, app).await{
        Ok(server) => server,
        Err(_msg) => return println!("Erro ao criar server axum")
    };

}
