use leptos::prelude::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

#[component]
pub fn WindowsBrowsePatientEntryPage() -> impl IntoView {
    let (sort_by, set_sort_by) = signal("requested_at".to_string());
    let patient_data = Resource::new(move || sort_by.get(), |sort| get_patients_for_browse(sort));

    view! {
        <Title text="Windows Browse Patient Entry - SyncMed"/>
        <div class="px-6 py-6 lg:px-8">
            <h1 class="text-[28px] font-semibold text-black/85">"Patient Entries"</h1>
        </div>

        <div class="flex-1 overflow-auto px-6 pb-10 lg:px-8">
            <div class="space-y-5">
                <div class="flex flex-wrap items-end justify-between gap-4 border-b border-black/10 pb-4">
                    <div class="card w-[120px] border border-black/10 bg-white shadow-md">
                        <div class="card-body items-center gap-1 p-3">
                            <p class="text-[10px] uppercase tracking-wide text-black/45">"Total session"</p>
                            <p class="text-6xl font-normal leading-none">
                                {move || {
                                    patient_data
                                        .get()
                                        .and_then(|res| res.ok())
                                        .map(|res| res.total_count.to_string())
                                        .unwrap_or_else(|| "0".to_string())
                                }}
                            </p>
                        </div>
                    </div>

                    <div class="flex w-full max-w-[430px] items-end gap-3">
                        <div class="w-28">
                            <label class="mb-1 block text-sm text-black/80">"Sort by"</label>
                            <select
                                class="select select-sm w-full rounded border-black/10 bg-white text-black/80"
                                prop:value=move || sort_by.get()
                                on:change=move |ev| set_sort_by.set(event_target_value(&ev))
                            >
                                <option value="name">"Name"</option>
                                <option value="age">"Age"</option>
                                <option value="gender">"Gender"</option>
                                <option value="status">"Status"</option>
                                <option value="requested_at">"Requested Time"</option>
                                <option value="filled_at">"Completed Time"</option>
                            </select>
                        </div>
                        <div class="flex-1">
                            <label class="mb-1 block text-sm text-black/80">"Search"</label>
                            <input
                                type="text"
                                placeholder="Search by name"
                                class="input input-sm w-full rounded border-black/10 bg-white text-black"
                            />
                        </div>
                    </div>
                </div>

                <div class="overflow-x-auto rounded-lg">
                    <table class="table table-sm w-full text-[13px] text-black/85">
                        <thead class="text-black/90">
                            <tr>
                                <th>"Name"</th>
                                <th>"Age"</th>
                                <th>"Gender"</th>
                                <th>"Status"</th>
                                <th>"Sent Time"</th>
                                <th>"Filled Time"</th>
                                <th>"Interact"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <Suspense fallback=move || view! { <tr><td colspan="7" class="py-4 text-center text-black/60">"Loading..."</td></tr> }>
                                {move || match patient_data.get() {
                                    Some(Ok(res)) => {
                                        if res.items.is_empty() {
                                            view! { <tr><td colspan="7" class="py-4 text-center text-black/60">"No patient entries found."</td></tr> }.into_any()
                                        } else {
                                            res.items
                                                .into_iter()
                                                .map(|item| {
                                                    view! {
                                                        <PatientRow
                                                            patient_key=item.patient_key
                                                            name=item.name
                                                            age=item.age
                                                            gender=item.gender
                                                            status=item.status
                                                            sent=item.requested_at
                                                            finish=item.filled_at
                                                        />
                                                    }
                                                })
                                                .collect_view()
                                                .into_any()
                                        }
                                    }
                                    Some(Err(err)) => view! {
                                        <tr>
                                            <td colspan="7" class="py-4 text-center text-red-600">
                                                {format!("Failed to load entries: {err}")}
                                            </td>
                                        </tr>
                                    }
                                        .into_any(),
                                    None => view! { <tr><td colspan="7" class="py-4 text-center text-black/60">"Loading..."</td></tr> }.into_any(),
                                }}
                            </Suspense>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PatientRow(
    patient_key: String,
    name: String,
    age: String,
    gender: String,
    status: String,
    sent: String,
    finish: String,
) -> impl IntoView {
    let detail_error = RwSignal::new(String::new());
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &patient_key;
    view! {
        <tr class="border-black/5">
            <td>{name}</td>
            <td>{age}</td>
            <td>{gender}</td>
            <td>{status.clone()}</td>
            <td>{sent}</td>
            <td>{finish}</td>
            <td>
                <button
                    type="button"
                    class="btn btn-xs min-h-6 h-6 border-none bg-[#005fb8] px-3 text-white hover:bg-[#0051a0]"
                    on:click=move |_| {
                        detail_error.set(String::new());

                        #[cfg(target_arch = "wasm32")]
                        {
                            let status_now = status.clone();
                            let key_now = patient_key.clone();
                            leptos::task::spawn_local(async move {
                                if status_now == "filled" {
                                    let _ = mark_patient_processed(key_now.clone()).await;
                                }
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        let _ = storage.set_item("last_patient_key", &key_now);
                                    }
                                }
                                if let Some(window) = web_sys::window() {
                                    let _ = window.location()
                                        .set_href(&format!("/windows/card-details?patient-id={key_now}"));
                                }
                            });
                        }
                    }
                >
                    "Detail"
                </button>
                <Show when=move || !detail_error.get().is_empty()>
                    <p class="text-[10px] text-red-600">{move || detail_error.get()}</p>
                </Show>
            </td>
        </tr>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientBrowseRow {
    pub patient_key: String,
    pub name: String,
    pub age: String,
    pub gender: String,
    pub status: String,
    pub requested_at: String,
    pub filled_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientBrowseResponse {
    pub total_count: i64,
    pub items: Vec<PatientBrowseRow>,
}

#[server(GetPatientsForBrowse, "/api")]
pub async fn get_patients_for_browse(
    sort_by: String,
) -> Result<PatientBrowseResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, models::PatientCase, schema::patient::dsl as patient_dsl};
        use axum::Extension;
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

        let patients: Vec<PatientCase> = match sort_by.as_str() {
            "name" => {
                patient_dsl::patient
                    .order(patient_dsl::name_snapshot.asc())
                    .load(&mut conn)
                    .await
            }
            "age" => {
                patient_dsl::patient
                    .order(patient_dsl::age_snapshot.asc())
                    .load(&mut conn)
                    .await
            }
            "gender" => {
                patient_dsl::patient
                    .order(patient_dsl::gender_snapshot.asc())
                    .load(&mut conn)
                    .await
            }
            "status" => {
                patient_dsl::patient
                    .order(patient_dsl::status.asc())
                    .load(&mut conn)
                    .await
            }
            "filled_at" => {
                patient_dsl::patient
                    .order(patient_dsl::filled_at.asc())
                    .load(&mut conn)
                    .await
            }
            "requested_at" => {
                patient_dsl::patient
                    .order(patient_dsl::requested_at.asc())
                    .load(&mut conn)
                    .await
            }
            _ => {
                patient_dsl::patient
                    .order(patient_dsl::requested_at.asc())
                    .load(&mut conn)
                    .await
            }
        }
        .map_err(|e| ServerFnError::new(format!("list query failed: {e}")))?;

        let items: Vec<PatientBrowseRow> = patients
            .into_iter()
            .map(|p| PatientBrowseRow {
                patient_key: p.patient_key,
                name: p.name_snapshot,
                age: p.age_snapshot.to_string(),
                gender: p.gender_snapshot,
                status: p.status,
                requested_at: p.requested_at.format("%Y-%m-%d %H:%M").to_string(),
                filled_at: p
                    .filled_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "-".to_string()),
            })
            .collect();
        let total_count = items.len() as i64;

        Ok(PatientBrowseResponse { total_count, items })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_patients_for_browse is only available on the server".to_string(),
        ))
    }
}

#[server(MarkPatientProcessed, "/api")]
pub async fn mark_patient_processed(patient_key: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, schema::patient::dsl as patient_dsl};
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

        diesel::update(
            patient_dsl::patient
                .filter(patient_dsl::patient_key.eq(patient_key))
                .filter(patient_dsl::status.eq("filled")),
        )
        .set((
            patient_dsl::status.eq("processed"),
            patient_dsl::modified_at.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::new(format!("update status failed: {e}")))?;

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "mark_patient_processed is only available on the server".to_string(),
        ))
    }
}
