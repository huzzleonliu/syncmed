use axum::{
    Json, Router, http::StatusCode, routing::post,
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
    medications: Option<Vec<CaseMedication>>,
}

#[derive(Debug, Clone, Serialize)]
struct CaseMedication {
    id: i32,
    patient_id: i32,
    med_name: String,
    dose: String,
    frequency: String,
    start_date: Option<String>,
    end_date: Option<String>,
    notes: Option<String>,
    created_at: String,
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
    let medications = match_mentioned_medications(message);

    Ok(Json(ChatResponse {
        reply,
        patient_id: payload.patient_id,
        timestamp: Utc::now().to_rfc3339(),
        medications,
    }))
}

fn match_mentioned_medications(message: &str) -> Option<Vec<CaseMedication>> {
    let lowered_message = message.to_lowercase();
    let matched: Vec<CaseMedication> = demo_medications()
        .into_iter()
        .filter(|med| lowered_message.contains(&med.med_name.to_lowercase()))
        .collect();

    if matched.is_empty() {
        None
    } else {
        Some(matched)
    }
}

fn demo_medications() -> Vec<CaseMedication> {
    vec![
        CaseMedication {
            id: 1,
            patient_id: 1,
            med_name: "Paracet".to_string(),
            dose: "500mg".to_string(),
            frequency: "twice a day".to_string(),
            start_date: Some("2026-03-10".to_string()),
            end_date: None,
            notes: Some("Take after meal, test note A".to_string()),
            created_at: "2026-03-16T09:00:00Z".to_string(),
        },
        CaseMedication {
            id: 2,
            patient_id: 1,
            med_name: "Ibuprofen".to_string(),
            dose: "200mg".to_string(),
            frequency: "once a day".to_string(),
            start_date: Some("2026-03-01".to_string()),
            end_date: Some("2026-03-20".to_string()),
            notes: Some("Short-term pain relief, test note B".to_string()),
            created_at: "2026-03-16T09:05:00Z".to_string(),
        },
        CaseMedication {
            id: 3,
            patient_id: 1,
            med_name: "Amoxicillin".to_string(),
            dose: "250mg".to_string(),
            frequency: "three times a day".to_string(),
            start_date: Some("2026-03-12".to_string()),
            end_date: Some("2026-03-19".to_string()),
            notes: Some("Finish full course, test note C".to_string()),
            created_at: "2026-03-16T09:10:00Z".to_string(),
        },
    ]
}
