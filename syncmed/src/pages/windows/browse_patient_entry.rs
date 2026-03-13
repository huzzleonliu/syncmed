use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
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
                                <option value="report_status">"Status"</option>
                                <option value="requested_at">"Requested Time"</option>
                                <option value="completed_at">"Completed Time"</option>
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
                                <th>"Report Status"</th>
                                <th>"Sent Time"</th>
                                <th>"Finish Time"</th>
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
                                                            name=item.name
                                                            age=item.age
                                                            gender=item.gender
                                                            status=item.report_status
                                                            sent=item.requested_at
                                                            finish=item.completed_at
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
                                                {format!("Failed to load patients: {err}")}
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
    name: String,
    age: String,
    gender: String,
    status: String,
    sent: String,
    finish: String,
) -> impl IntoView {
    let is_finished = status == "finished";
    view! {
        <tr class="border-black/5">
            <td>{name}</td>
            <td>{age}</td>
            <td>{gender}</td>
            <td>{status}</td>
            <td>{sent}</td>
            <td>{finish}</td>
            <td>
                {if is_finished {
                    view! {
                        <A href="/windows/card-details" attr:class="btn btn-xs min-h-6 h-6 border-none bg-[#005fb8] px-3 text-white hover:bg-[#0051a0]">
                            "Detail"
                        </A>
                    }
                        .into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            disabled
                            class="btn btn-xs h-6 min-h-6 cursor-not-allowed border-none bg-base-300 px-3 text-base-content/60"
                        >
                            "Detail"
                        </button>
                    }
                        .into_any()
                }}
            </td>
        </tr>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientBrowseRow {
    pub name: String,
    pub age: String,
    pub gender: String,
    pub report_status: String,
    pub requested_at: String,
    pub completed_at: String,
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
        use crate::db::{DbPool, models::Patient, schema::patients::dsl as patients_dsl};
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

        let patients: Vec<Patient> = match sort_by.as_str() {
            "name" => {
                patients_dsl::patients
                    .order(patients_dsl::name.asc())
                    .load(&mut conn)
                    .await
            }
            "age" => {
                patients_dsl::patients
                    .order(patients_dsl::age.asc())
                    .load(&mut conn)
                    .await
            }
            "gender" => {
                patients_dsl::patients
                    .order(patients_dsl::gender.asc())
                    .load(&mut conn)
                    .await
            }
            "report_status" => {
                patients_dsl::patients
                    .order(patients_dsl::report_status.asc())
                    .load(&mut conn)
                    .await
            }
            "completed_at" => {
                patients_dsl::patients
                    .order(patients_dsl::completed_at.asc())
                    .load(&mut conn)
                    .await
            }
            "requested_at" => {
                patients_dsl::patients
                    .order(patients_dsl::requested_at.asc())
                    .load(&mut conn)
                    .await
            }
            _ => {
                patients_dsl::patients
                    .order(patients_dsl::requested_at.asc())
                    .load(&mut conn)
                    .await
            }
        }
        .map_err(|e| ServerFnError::new(format!("list query failed: {e}")))?;

        let items: Vec<PatientBrowseRow> = patients
            .into_iter()
            .map(|p| PatientBrowseRow {
                name: p.name,
                age: p.age.to_string(),
                gender: p.gender,
                report_status: p.report_status,
                requested_at: p.requested_at.format("%Y-%m-%d %H:%M").to_string(),
                completed_at: p
                    .completed_at
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
