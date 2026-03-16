use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use serde::{Deserialize, Serialize};

#[component]
pub fn WindowsCardDetailsPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });

    let details = Resource::new(
        move || patient_key.get(),
        |key| async move {
            if key.trim().is_empty() {
                return Err(ServerFnError::new("Missing patient-id in URL"));
            }
            get_patient_card_details(key).await
        },
    );

    view! {
        <Title text="Windows Card Details - SyncMed"/>
        <div class="px-6 py-6 lg:px-8">
            <h1 class="text-[28px] font-semibold text-black/85">"Chat Details"</h1>
        </div>

        <div class="flex-1 overflow-auto px-6 pb-8 lg:px-8">
            <Suspense fallback=move || view! { <p class="text-sm text-black/60">"Loading details..."</p> }>
                {move || match details.get() {
                    Some(Ok(data)) => view! {
                        <div class="space-y-6">
                            <div class="border-b border-black/10 pb-4">
                                <h2 class="mb-4 text-2xl font-semibold text-black/85">"Patient Info"</h2>
                                <div class="grid grid-cols-1 gap-3 text-sm text-black/85 sm:grid-cols-3">
                                    <p><span class="font-bold">"Name "</span>{data.name_snapshot.clone()}</p>
                                    <p><span class="font-bold">"Age "</span>{data.age_snapshot.clone()}</p>
                                    <p><span class="font-bold">"Gender "</span>{data.gender_snapshot.clone()}</p>
                                </div>
                            </div>

                            <div class="border-b border-black/10 pb-4">
                                <h2 class="mb-4 text-2xl font-semibold text-black/85">"Medication List"</h2>
                                <div class="min-h-0 flex-1 overflow-auto rounded-lg">
                                    <table class="table table-sm w-full text-[13px] text-black/85">
                                        <thead class="text-black/90">
                                            <tr>
                                                <th>"Index"</th>
                                                <th>"Medication Name"</th>
                                                <th>"Dose"</th>
                                                <th>"Frequency"</th>
                                                <th>"Taking Period"</th>
                                                <th>"Notes"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {if data.medications.is_empty() {
                                                view! {
                                                    <tr>
                                                        <td colspan="6" class="py-4 text-center text-black/60">
                                                            "No medications yet."
                                                        </td>
                                                    </tr>
                                                }.into_any()
                                            } else {
                                                data.medications
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(idx, med)| {
                                                        view! {
                                                            <tr class="border-black/5">
                                                                <td>{idx + 1}</td>
                                                                <td>{med.med_name}</td>
                                                                <td>{med.dose}</td>
                                                                <td>{med.frequency}</td>
                                                                <td>{med.period}</td>
                                                                <td>{med.notes}</td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }}
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <div>
                                <h2 class="mb-4 text-2xl font-semibold text-black/85">"Chat History"</h2>
                                <div class="space-y-3 min-h-0 flex-1 overflow-auto rounded-lg">
                                    {if data.chat_messages.is_empty() {
                                        view! {
                                            <div class="rounded border border-black/10 bg-white/70 p-4 text-[13px] text-black/60">
                                                "No chat messages yet."
                                            </div>
                                        }.into_any()
                                    } else {
                                        data.chat_messages
                                            .into_iter()
                                            .map(|msg| {
                                                view! {
                                                    <div class="rounded border border-black/10 bg-white/70 p-4 text-[13px] text-black/80">
                                                        <div class="grid grid-cols-[90px,1fr] gap-4">
                                                            <p class="font-medium">{msg.sender_label}</p>
                                                            <p>{msg.content_text}</p>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }}
                                </div>
                            </div>

                            <div class="flex items-center justify-between pt-1">
                                <A href="/windows/browse-patient-entry" attr:class="btn btn-ghost btn-sm text-black/70">"Back"</A>
                            </div>
                        </div>
                    }.into_any(),
                    Some(Err(err)) => view! {
                        <div class="space-y-4">
                            <p class="text-sm text-red-600">{format!("Failed to load details: {err}")}</p>
                            <A href="/windows/browse-patient-entry" attr:class="btn btn-sm">"Back to list"</A>
                        </div>
                    }.into_any(),
                    None => view! { <p class="text-sm text-black/60">"Loading details..."</p> }.into_any(),
                }}
            </Suspense>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MedicationView {
    pub med_name: String,
    pub dose: String,
    pub frequency: String,
    pub period: String,
    pub notes: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessageView {
    pub sender_label: String,
    pub content_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientCardDetailsResponse {
    pub patient_key: String,
    pub name_snapshot: String,
    pub age_snapshot: String,
    pub gender_snapshot: String,
    pub status: String,
    pub requested_at: String,
    pub filled_at: String,
    pub medications: Vec<MedicationView>,
    pub chat_messages: Vec<ChatMessageView>,
}

#[server(GetPatientCardDetails, "/api")]
pub async fn get_patient_card_details(
    patient_key: String,
) -> Result<PatientCardDetailsResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool,
            models::{CaseChatMessage, CaseMedication, PatientCase},
            schema::{
                case_chat_messages::dsl as chat_dsl, case_medications::dsl as meds_dsl,
                patient::dsl as patient_dsl,
            },
        };
        use axum::Extension;
        use chrono::Utc;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let mut patient: PatientCase = patient_dsl::patient
            .filter(patient_dsl::patient_key.eq(&patient_key))
            .first(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient query failed: {e}")))?;

        if patient.status == "filled" {
            diesel::update(patient_dsl::patient.filter(patient_dsl::id.eq(patient.id)))
                .set((
                    patient_dsl::status.eq("processed"),
                    patient_dsl::modified_at.eq(Some(Utc::now().naive_utc())),
                ))
                .execute(&mut conn)
                .await
                .map_err(|e| ServerFnError::new(format!("update status failed: {e}")))?;

            patient.status = "processed".to_string();
        }

        let medications: Vec<CaseMedication> = meds_dsl::case_medications
            .filter(meds_dsl::patient_id.eq(patient.id))
            .order(meds_dsl::created_at.asc())
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("medications query failed: {e}")))?;

        let chat_messages: Vec<CaseChatMessage> = chat_dsl::case_chat_messages
            .filter(chat_dsl::patient_id.eq(patient.id))
            .order(chat_dsl::created_at.asc())
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("chat query failed: {e}")))?;

        Ok(PatientCardDetailsResponse {
            patient_key: patient.patient_key,
            name_snapshot: patient.name_snapshot,
            age_snapshot: patient.age_snapshot.to_string(),
            gender_snapshot: patient.gender_snapshot,
            status: patient.status,
            requested_at: patient.requested_at.format("%Y-%m-%d %H:%M").to_string(),
            filled_at: patient
                .filled_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "-".to_string()),
            medications: medications
                .into_iter()
                .map(|med| MedicationView {
                    med_name: med.med_name,
                    dose: med.dose,
                    frequency: med.frequency,
                    period: format!(
                        "{} -> {}",
                        med.start_date
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "-".to_string()),
                        med.end_date
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "-".to_string())
                    ),
                    notes: med.notes.unwrap_or_else(|| "-".to_string()),
                })
                .collect(),
            chat_messages: chat_messages
                .into_iter()
                .map(|msg| ChatMessageView {
                    sender_label: match msg.sender_type.as_str() {
                        "patient" => "Patient",
                        "bot" => "Chat Bot",
                        _ => "System",
                    }
                    .to_string(),
                    content_text: msg.content_text,
                })
                .collect(),
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_patient_card_details is only available on the server".to_string(),
        ))
    }
}
