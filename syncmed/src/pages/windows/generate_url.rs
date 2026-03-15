use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

#[component]
pub fn WindowsGenerateUrlPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (age, set_age) = signal(String::new());
    let (gender, set_gender) = signal(String::new());
    let (generated_url, set_generated_url) = signal(String::new());
    let (error_text, set_error_text) = signal(String::new());

    view! {
        <Title text="Windows Generate URL - SyncMed"/>
        <div class="px-6 py-6 lg:px-8">
            <h1 class="text-[28px] font-semibold text-black/85">"Generate Patient URL"</h1>
        </div>

        <div class="flex-1 overflow-auto px-6 pb-10 lg:px-8">
            <div class="mx-auto w-full max-w-[620px] space-y-10">
                <div>
                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Create Entry"</h2>
                    <div class="space-y-3">
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"Name"</label>
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"Age"</label>
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || age.get()
                                on:input=move |ev| set_age.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"Gender"</label>
                            <input
                                type="text"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || gender.get()
                                on:input=move |ev| set_gender.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                    <div class="mt-4 flex justify-center">
                        <button
                            type="button"
                            class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]"
                            on:click=move |_| {
                                set_error_text.set(String::new());
                                let req_name = name.get_untracked();
                                let req_age = age.get_untracked();
                                let req_gender = gender.get_untracked();

                                #[cfg(target_arch = "wasm32")]
                                {
                                    leptos::task::spawn_local(async move {
                                        match generate_patient_url(req_name, req_age, req_gender).await {
                                            Ok(payload) => set_generated_url.set(payload.url),
                                            Err(err) => set_error_text.set(err.to_string()),
                                        }
                                    });
                                }

                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    let _ = (&set_generated_url, req_name, req_age, req_gender);
                                    set_error_text
                                        .set("Generate URL is available after hydration.".to_string());
                                }
                            }
                        >
                            "generate URL"
                        </button>
                    </div>
                    <Show when=move || !error_text.get().is_empty()>
                        <p class="mt-2 text-sm text-red-600">{move || error_text.get()}</p>
                    </Show>
                </div>

                <div>
                    <h2 class="mb-4 text-2xl font-semibold text-black/85">"Send Link"</h2>
                    <div class="space-y-3">
                        <div>
                            <textarea
                                placeholder="Generated URL appears here"
                                class="textarea h-28 w-full rounded border-black/10 bg-white/80 text-black"
                                prop:value=move || generated_url.get()
                                readonly
                            ></textarea>
                        </div>
                        <div class="flex justify-center">
                            <button
                                type="button"
                                class="btn btn-xs min-w-28 border-none bg-[#005fb8] text-white hover:bg-[#0051a0] disabled:bg-base-300 disabled:text-base-content/60"
                                disabled=move || generated_url.get().is_empty()
                                on:click=move |_| {
                                    let url = generated_url.get_untracked();
                                    if url.is_empty() {
                                        return;
                                    }
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            let clipboard = window.navigator().clipboard();
                                            let _ = clipboard.write_text(&url);
                                        }
                                    }
                                }
                            >
                                "copy URL"
                            </button>
                        </div>
                        <div>
                            <label class="mb-1 block text-sm text-black/80">"e-mail address"</label>
                            <input
                                type="email"
                                placeholder="Text"
                                class="input input-sm w-full rounded border-black/10 bg-white/80 text-black"
                            />
                        </div>
                        <div class="flex items-center justify-between pt-1">
                            <A href="/windows/browse-patient-entry" attr:class="btn btn-xs min-w-40 border-none bg-[#005fb8] text-white hover:bg-[#0051a0]">
                                "send URL though email"
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateUrlResponse {
    pub url: String,
    pub patient_key: String,
}

#[server(GeneratePatientUrl, "/api")]
pub async fn generate_patient_url(
    name: String,
    age: String,
    gender: String,
) -> Result<GenerateUrlResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, models::NewPatientCase, schema::patient::dsl as patient_dsl};
        use axum::Extension;
        use chrono::Utc;
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

        let new_patient = NewPatientCase {
            patient_key: patient_key.clone(),
            doctor_user_id: None,
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

        Ok(GenerateUrlResponse {
            url: format!("https://app.syncmed.no/app?patient-id={patient_key}"),
            patient_key,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "generate_patient_url is only available on the server".to_string(),
        ))
    }
}
