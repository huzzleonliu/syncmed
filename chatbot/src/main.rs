use axum::{
    Json, Router, http::StatusCode, response::IntoResponse, routing::{get, post},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    patient_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    reply: String,
    patient_id: Option<String>,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/chat", post(chat))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    println!("chatbot listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind chatbot port failed");
    axum::serve(listener, app)
        .await
        .expect("chatbot server crashed");
}

async fn chat(Json(payload): Json<ChatRequest>) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    let message = payload.message.trim();
    if message.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "message cannot be empty".to_string()));
    }
    let reply = format!("已收到“{}”", message);

    Ok(Json(ChatResponse {
        reply,
        patient_id: payload.patient_id,
        timestamp: Utc::now().to_rfc3339(),
    }))
}
