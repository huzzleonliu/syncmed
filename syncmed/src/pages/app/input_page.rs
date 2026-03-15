use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/7b60b857-7bc2-4087-acab-4d5c37a8a322";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/73c194b0-97fd-42e5-a422-370d22b926ef";

#[component]
pub fn AppInputPage() -> impl IntoView {
    view! {
        <Title text="App Input Page - SyncMed"/>
        <main class="h-dvh bg-custom-subtle-background text-custom-foreground flex flex-col">
            <header class="border-b border-custom-border bg-custom-card">
                <div class="mx-auto flex h-[85px] w-full max-w-[1280px] items-center justify-between px-4 md:px-6">
                    <div class="flex items-center gap-3">
                        <img src=LOGO_GROUP_URL alt="SyncMed Logo" class="h-8 w-8 object-contain"/>
                        <span class="text-xl font-bold text-custom-primary">"SyncMed"</span>
                    </div>

                    <div class="hidden items-center gap-3 sm:flex">
                        <div class="avatar">
                            <div class="w-8 rounded-full">
                                <img src=AVATAR_IMAGE_URL alt="Nurse avatar"/>
                            </div>
                        </div>
                        <div class="text-sm leading-5 text-custom-accent-foreground">
                            <p class="font-bold">"Nurse"</p>
                            <p class="font-light">"Hilde.C@gmail.com"</p>
                        </div>
                    </div>
                </div>
            </header>

            <section class="mx-auto w-full max-w-[1280px] px-4 py-8 md:px-6 md:py-10 flex-1 min-h-0">
                <div class="grid gap-6 lg:grid-cols-[1fr_500px]">
                    <CreateEntryCard/>
                    <PatientListCard/>
                </div>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn CreateEntryCard() -> impl IntoView {
    view! {
        <div class=format!("{} {} {} {} ", 
            // Layout
        "bg-custom-background shadow-sm",
        "h-full",
        // Border   
        "border border-custom-border ",
        // Appearance
        "card rounded-xl")>
            <div class="card-body gap-4 p-5 h-full md:p-6">
                <h2 class="text-lg font-bold text-custom-foreground">"Create Entry"</h2>

                <label class="form-control">
                    <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Patient Name"</span>
                    <input
                        type="text"
                        placeholder="Patient Name"
                        class="input input-bordered h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none"
                    />
                </label>

                <div class="grid gap-3 sm:grid-cols-2">
                    <label class="form-control">
                        <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Age"</span>
                        <input
                            type="number"
                            min="0"
                            placeholder="Age"
                            class="input input-bordered h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none"
                        />
                    </label>

                    <label class="form-control">
                        <span class="label-text mb-2 text-base font-medium text-custom-foreground">"Gender"</span>
                        <select class="select select-bordered h-9 min-h-9 w-full border-custom-ring bg-custom-background text-custom-accent-foreground focus:outline-none">
                            <option value="female">"Female"</option>
                            <option value="male">"Male"</option>
                            <option value="other">"Other"</option>
                        </select>
                    </label>
                </div>

                <div class="mt-2 flex justify-end">
                    <A href="/app/confirm-identity" attr:class="btn btn-primary h-9 min-h-9 px-6">"Create"</A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PatientListCard() -> impl IntoView {
    let patient_list = Resource::new(|| (), |_| get_app_patient_list());

    view! {
        <div class="card rounded-xl border border-custom-border bg-custom-background shadow-sm">
            <div class="border-b border-custom-border px-5 py-3 md:px-6">
                <h3 class="text-lg font-bold text-custom-foreground">"patient list"</h3>
            </div>

            <div class="overflow-x-auto">
                <table class="table table-zebra">
                    <thead>
                        <tr class="text-base font-semibold text-custom-foreground">
                            <th>"Name"</th>
                            <th>"Age"</th>
                            <th>"Gender"</th>
                            <th>"entry creator"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <Suspense fallback=move || view! {
                            <tr class="text-base text-custom-foreground">
                                <td colspan="5" class="text-center text-custom-muted-foreground">"Loading..."</td>
                            </tr>
                        }>
                            {move || match patient_list.get() {
                                Some(Ok(items)) => {
                                    if items.is_empty() {
                                        view! {
                                            <tr class="text-base text-custom-foreground">
                                                <td colspan="5" class="text-center text-custom-muted-foreground">
                                                    "No patient entries"
                                                </td>
                                            </tr>
                                        }.into_any()
                                    } else {
                                        items
                                            .into_iter()
                                            .map(|item| {
                                                view! {
                                                    <PatientRow
                                                        patient_key=item.patient_key
                                                        name=item.name_snapshot
                                                        age=item.age_snapshot
                                                        gender=item.gender_snapshot
                                                        creator=item.entry_creator
                                                    />
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }
                                }
                                Some(Err(err)) => view! {
                                    <tr class="text-base text-custom-foreground">
                                        <td colspan="5" class="text-center text-red-600">
                                            {format!("Failed to load patient list: {err}")}
                                        </td>
                                    </tr>
                                }.into_any(),
                                None => view! {
                                    <tr class="text-base text-custom-foreground">
                                        <td colspan="5" class="text-center text-custom-muted-foreground">"Loading..."</td>
                                    </tr>
                                }.into_any(),
                            }}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn PatientRow(
    patient_key: String,
    name: String,
    age: i32,
    gender: String,
    creator: String,
) -> impl IntoView {
    view! {
        <tr class="text-base text-custom-foreground">
            <td>{name}</td>
            <td>{age}</td>
            <td>{gender}</td>
            <td>{creator}</td>
            <td class="text-right">
                <A
                    href=format!("/app/confirm-identity?patient-id={patient_key}")
                    attr:class="btn btn-primary h-9 min-h-9 px-4"
                >
                    "Choose"
                </A>
            </td>
        </tr>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppPatientRow {
    pub patient_key: String,
    pub name_snapshot: String,
    pub age_snapshot: i32,
    pub gender_snapshot: String,
    pub entry_creator: String,
}

#[server(GetAppPatientList, "/api")]
pub async fn get_app_patient_list() -> Result<Vec<AppPatientRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, schema::{patient::dsl as patient_dsl, users::dsl as users_dsl}};
        use axum::Extension;
        use diesel::ExpressionMethods;
        use diesel::JoinOnDsl;
        use diesel::NullableExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<DbPool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("pool extract failed: {e}")))?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| ServerFnError::new(format!("pool get failed: {e}")))?;

        let rows: Vec<(String, String, i32, String, Option<String>, Option<String>)> = patient_dsl::patient
            .left_join(users_dsl::users.on(users_dsl::id.nullable().eq(patient_dsl::doctor_user_id)))
            .select((
                patient_dsl::patient_key,
                patient_dsl::name_snapshot,
                patient_dsl::age_snapshot,
                patient_dsl::gender_snapshot,
                users_dsl::display_name.nullable(),
                users_dsl::role.nullable(),
            ))
            .order((patient_dsl::modified_at.desc(), patient_dsl::requested_at.desc()))
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient list query failed: {e}")))?;

        Ok(rows
            .into_iter()
            .map(|(patient_key, name_snapshot, age_snapshot, gender_snapshot, creator_name, creator_role)| {
                let entry_creator = match (creator_name, creator_role) {
                    (Some(name), Some(role)) => format!("{name} ({role})"),
                    _ => "-".to_string(),
                };
                AppPatientRow {
                    patient_key,
                    name_snapshot,
                    age_snapshot,
                    gender_snapshot,
                    entry_creator,
                }
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_app_patient_list is only available on the server".to_string(),
        ))
    }
}
