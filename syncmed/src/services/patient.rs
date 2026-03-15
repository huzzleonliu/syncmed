use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePatientEntryResponse {
    pub url: String,
    pub patient_key: String,
}

#[server(CreatePatientEntry, "/api")]
pub async fn create_patient_entry(
    name: String,
    age: String,
    gender: String,
) -> Result<CreatePatientEntryResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool, models::NewPatientCase,
            schema::{patient::dsl as patient_dsl, users::dsl as users_dsl},
        };
        use axum::Extension;
        use chrono::Utc;
        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;
        use sha2::{Digest, Sha256};

        let name = name.trim().to_string();
        let gender = gender.trim().to_string();
        let age_num = age
            .trim()
            .parse::<i32>()
            .map_err(|_| ServerFnError::new("Age must be a valid number"))?;

        if name.is_empty() || gender.is_empty() {
            return Err(ServerFnError::new("Name and gender are required"));
        }

        let now = Utc::now().naive_utc();
        let nonce: u64 = rand::random();
        let source = format!(
            "{name}|{age_num}|{gender}|{}|{nonce}",
            now.and_utc().timestamp()
        );

        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        let patient_key = format!("{:x}", hasher.finalize());

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;
        let lucas_doctor_id = users_dsl::users
            .filter(users_dsl::display_name.eq("Lucas Berg"))
            .filter(users_dsl::role.eq("doctor"))
            .select(users_dsl::id)
            .first::<i32>(&mut conn)
            .await
            .map_err(|e| {
                ServerFnError::new(format!("query default doctor (Lucas Berg) failed: {e}"))
            })?;

        let new_patient = NewPatientCase {
            patient_key: patient_key.clone(),
            doctor_user_id: Some(lucas_doctor_id),
            name_snapshot: name,
            age_snapshot: age_num,
            gender_snapshot: gender,
            status: "sent".to_string(),
            requested_at: now,
            filled_at: None,
            modified_at: Some(now),
        };

        diesel::insert_into(patient_dsl::patient)
            .values(&new_patient)
            .execute(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("insert patient failed: {e}")))?;

        Ok(CreatePatientEntryResponse {
            url: format!("https://app.syncmed.no/app?patient-id={patient_key}"),
            patient_key,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "create_patient_entry is only available on the server".to_string(),
        ))
    }
}
