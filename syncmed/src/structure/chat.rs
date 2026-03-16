use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[allow(dead_code)]
pub struct CaseChatMessageDraft {
    pub id: Option<i32>,
    pub patient_id: Option<i32>,
    pub sender_type: String,
    pub content_text: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatbotRequest {
    pub message: String,
    pub patient_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatbotResponse {
    pub reply: String,
    pub timestamp: String,
}
