use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::structure::chat::{CaseChatMessageDraft, CaseMedicationPayload};

pub fn merge_case_medications(
    local: &mut Vec<CaseMedicationPayload>,
    incoming: Vec<CaseMedicationPayload>,
) {
    for incoming_med in incoming {
        if let Some(existing) = local
            .iter_mut()
            .find(|m| m.med_name.eq_ignore_ascii_case(&incoming_med.med_name))
        {
            // Keep med_name stable, replace other fields.
            existing.id = incoming_med.id;
            existing.patient_id = incoming_med.patient_id;
            existing.dose = incoming_med.dose;
            existing.frequency = incoming_med.frequency;
            existing.start_date = incoming_med.start_date;
            existing.end_date = incoming_med.end_date;
            existing.notes = incoming_med.notes;
            existing.created_at = incoming_med.created_at;
        } else {
            local.push(incoming_med);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadLocalDataResult {
    pub uploaded_chat_count: usize,
    pub upserted_medication_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBootstrapData {
    pub messages: Vec<CaseChatMessageDraft>,
    pub medications: Vec<CaseMedicationPayload>,
}

#[server(GetChatBootstrapData, "/api")]
pub async fn get_chat_bootstrap_data(
    patient_key: String,
) -> Result<ChatBootstrapData, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool,
            models::{CaseChatMessage, CaseMedication},
            schema::{
                case_chat_messages::dsl as chat_dsl, case_medications::dsl as meds_dsl,
                patient::dsl as patient_dsl,
            },
        };
        use axum::Extension;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        if patient_key.trim().is_empty() {
            return Ok(ChatBootstrapData {
                messages: Vec::new(),
                medications: Vec::new(),
            });
        }

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let patient_id: i32 = patient_dsl::patient
            .filter(patient_dsl::patient_key.eq(&patient_key))
            .select(patient_dsl::id)
            .first(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient query failed: {e}")))?;

        let db_messages: Vec<CaseChatMessage> = chat_dsl::case_chat_messages
            .filter(chat_dsl::patient_id.eq(patient_id))
            .order(chat_dsl::created_at.asc())
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("chat query failed: {e}")))?;

        let db_medications: Vec<CaseMedication> = meds_dsl::case_medications
            .filter(meds_dsl::patient_id.eq(patient_id))
            .order(meds_dsl::created_at.desc())
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("medications query failed: {e}")))?;

        Ok(ChatBootstrapData {
            messages: db_messages
                .into_iter()
                .map(|m| CaseChatMessageDraft {
                    id: Some(m.id),
                    patient_id: Some(m.patient_id),
                    sender_type: m.sender_type,
                    content_text: m.content_text,
                    created_at: m.created_at.format("%Y-%m-%d %H:%M").to_string(),
                })
                .collect(),
            medications: db_medications
                .into_iter()
                .map(|m| CaseMedicationPayload {
                    id: m.id,
                    patient_id: m.patient_id,
                    med_name: m.med_name,
                    dose: m.dose,
                    frequency: m.frequency,
                    start_date: m.start_date.map(|d| d.to_string()),
                    end_date: m.end_date.map(|d| d.to_string()),
                    notes: m.notes,
                    created_at: m.created_at.format("%Y-%m-%d %H:%M").to_string(),
                })
                .collect(),
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_chat_bootstrap_data is only available on the server".to_string(),
        ))
    }
}

#[cfg(target_arch = "wasm32")]
fn history_storage_key(patient_key: &str) -> String {
    let key = patient_key.trim();
    if key.is_empty() {
        "syncmed.chat_history.__anon__".to_string()
    } else {
        format!("syncmed.chat_history.{key}")
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_chat_history(patient_key: &str) -> Vec<CaseChatMessageDraft> {
    let Some(window) = web_sys::window() else {
        return Vec::new();
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return Vec::new();
    };
    let key = history_storage_key(patient_key);
    let Ok(Some(raw)) = storage.get_item(&key) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<CaseChatMessageDraft>>(&raw).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn save_chat_history(patient_key: &str, messages: &[CaseChatMessageDraft]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    if let Ok(raw) = serde_json::to_string(messages) {
        let _ = storage.set_item(&history_storage_key(patient_key), &raw);
    }
}

#[cfg(target_arch = "wasm32")]
fn medications_storage_key(patient_key: &str) -> String {
    let key = patient_key.trim();
    if key.is_empty() {
        "syncmed.medications.__anon__".to_string()
    } else {
        format!("syncmed.medications.{key}")
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_local_medications(patient_key: &str) -> Vec<CaseMedicationPayload> {
    let Some(window) = web_sys::window() else {
        return Vec::new();
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return Vec::new();
    };
    let key = medications_storage_key(patient_key);
    let Ok(Some(raw)) = storage.get_item(&key) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<CaseMedicationPayload>>(&raw).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub fn save_local_medications(patient_key: &str, medications: &[CaseMedicationPayload]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    if let Ok(raw) = serde_json::to_string(medications) {
        let _ = storage.set_item(&medications_storage_key(patient_key), &raw);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn bootstrap_chat_state(
    patient_key: String,
    messages_store: RwSignal<Vec<CaseChatMessageDraft>>,
    medications_store: RwSignal<Vec<CaseMedicationPayload>>,
) {
    leptos::task::spawn_local(async move {
        let local_messages = load_chat_history(&patient_key);
        let local_medications = load_local_medications(&patient_key);
        if !local_messages.is_empty() || !local_medications.is_empty() {
            messages_store.set(local_messages);
            medications_store.set(local_medications);
            return;
        }

        if let Ok(data) = get_chat_bootstrap_data(patient_key.clone()).await {
            messages_store.set(data.messages.clone());
            medications_store.set(data.medications.clone());
            save_chat_history(&patient_key, &data.messages);
            save_local_medications(&patient_key, &data.medications);
        } else {
            messages_store.set(load_chat_history(&patient_key));
            medications_store.set(load_local_medications(&patient_key));
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn clear_local_chat_bundle(patient_key: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(Some(storage)) = window.local_storage() else {
        return;
    };
    let _ = storage.remove_item(&history_storage_key(patient_key));
    let _ = storage.remove_item(&medications_storage_key(patient_key));
}

#[server(UploadLocalChatAndMedications, "/api")]
pub async fn upload_local_chat_and_medications(
    patient_key: String,
    messages: Vec<CaseChatMessageDraft>,
    medications: Vec<CaseMedicationPayload>,
) -> Result<UploadLocalDataResult, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool,
            models::{CaseMedication, NewCaseChatMessage, NewCaseMedication},
            schema::{case_medications::dsl as meds_dsl, patient::dsl as patient_dsl},
        };
        use axum::Extension;
        use chrono::{NaiveDate, Utc};
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        if patient_key.trim().is_empty() {
            return Err(ServerFnError::new("Missing patient-id in URL"));
        }

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let patient_id: i32 = patient_dsl::patient
            .filter(patient_dsl::patient_key.eq(&patient_key))
            .select(patient_dsl::id)
            .first(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient query failed: {e}")))?;

        let mut uploaded_chat_count = 0_usize;
        for msg in messages {
            if msg.id.is_some() {
                continue;
            }
            let content = msg.content_text.trim().to_string();
            if content.is_empty() {
                continue;
            }
            let sender = if msg.sender_type == "bot" { "bot" } else { "patient" }.to_string();
            let new_message = NewCaseChatMessage {
                patient_id,
                sender_type: sender,
                content_text: content,
            };
            diesel::insert_into(crate::db::schema::case_chat_messages::dsl::case_chat_messages)
                .values(&new_message)
                .execute(&mut conn)
                .await
                .map_err(|e| ServerFnError::new(format!("insert chat failed: {e}")))?;
            uploaded_chat_count += 1;
        }

        let mut upserted_medication_count = 0_usize;
        for med in medications {
            let med_name = med.med_name.trim().to_string();
            if med_name.is_empty() {
                continue;
            }
            let parse_date = |s: Option<String>| {
                s.and_then(|v| NaiveDate::parse_from_str(v.trim(), "%Y-%m-%d").ok())
            };
            let start_date = parse_date(med.start_date);
            let end_date = parse_date(med.end_date);
            let dose = med.dose.trim().to_string();
            let frequency = med.frequency.trim().to_string();
            let notes = med.notes.map(|n| n.trim().to_string()).filter(|n| !n.is_empty());

            let existing: Option<CaseMedication> = meds_dsl::case_medications
                .filter(meds_dsl::patient_id.eq(patient_id))
                .filter(meds_dsl::med_name.eq(&med_name))
                .first(&mut conn)
                .await
                .optional()
                .map_err(|e| ServerFnError::new(format!("medication query failed: {e}")))?;

            if let Some(row) = existing {
                diesel::update(meds_dsl::case_medications.filter(meds_dsl::id.eq(row.id)))
                    .set((
                        meds_dsl::dose.eq(dose),
                        meds_dsl::frequency.eq(frequency),
                        meds_dsl::start_date.eq(start_date),
                        meds_dsl::end_date.eq(end_date),
                        meds_dsl::notes.eq(notes),
                    ))
                    .execute(&mut conn)
                    .await
                    .map_err(|e| ServerFnError::new(format!("update medication failed: {e}")))?;
            } else {
                let new_med = NewCaseMedication {
                    patient_id,
                    med_name,
                    dose,
                    frequency,
                    start_date,
                    end_date,
                    notes,
                };
                diesel::insert_into(meds_dsl::case_medications)
                    .values(&new_med)
                    .execute(&mut conn)
                    .await
                    .map_err(|e| ServerFnError::new(format!("insert medication failed: {e}")))?;
            }
            upserted_medication_count += 1;
        }

        diesel::update(patient_dsl::patient.filter(patient_dsl::id.eq(patient_id)))
            .set(patient_dsl::modified_at.eq(Some(Utc::now().naive_utc())))
            .execute(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("update patient modified_at failed: {e}")))?;

        Ok(UploadLocalDataResult {
            uploaded_chat_count,
            upserted_medication_count,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (patient_key, messages, medications);
        Err(ServerFnError::new(
            "upload_local_chat_and_medications is only available on the server".to_string(),
        ))
    }
}
