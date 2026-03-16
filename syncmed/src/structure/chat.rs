use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub medications: Option<Vec<CaseMedicationPayload>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseMedicationPayload {
    pub id: i32,
    pub patient_id: i32,
    pub med_name: String,
    pub dose: String,
    pub frequency: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}
