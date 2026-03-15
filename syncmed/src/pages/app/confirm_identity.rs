use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use serde::{Deserialize, Serialize};

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/6f805919-02fb-42fa-bff3-e0bb62fe4a2b";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/37f0cd67-8cb9-4fcc-a62e-abf8704a6046";
const HELLO_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/00f7917e-d891-440b-960b-14150b9d2c4c";

#[component]
pub fn AppConfirmIdentityPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let identity = Resource::new(
        move || patient_key.get(),
        |key| async move {
            if key.trim().is_empty() {
                return Err(ServerFnError::new("Missing patient-id in URL"));
            }
            get_confirm_identity_data(key).await
        },
    );

    view! {
        <Title text="App Confirm Identity - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
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

            <section class="mx-auto w-full max-w-[1280px] px-4 py-8 md:px-6 md:py-10">
                <Suspense fallback=move || view! { <p class="text-center text-custom-muted-foreground">"Loading..."</p> }>
                    {move || match identity.get() {
                        Some(Ok(data)) => view! {
                            <div class="mx-auto flex max-w-[1108px] flex-col items-center justify-between gap-8 lg:flex-row lg:items-center">
                                <IdentitySummary
                                    name=data.name_snapshot
                                    gender=data.gender_snapshot
                                    age=data.age_snapshot.to_string()
                                />
                                <ConfirmPanel patient_key=data.patient_key/>
                            </div>
                        }.into_any(),
                        Some(Err(err)) => view! {
                            <div class="mx-auto max-w-[820px] rounded-xl border border-red-300 bg-red-50 p-6 text-center">
                                <p class="text-red-600">{format!("Failed to load patient info: {err}")}</p>
                                <div class="mt-4">
                                    <A href="/app/input-page" attr:class="btn btn-outline">"Back"</A>
                                </div>
                            </div>
                        }.into_any(),
                        None => view! { <p class="text-center text-custom-muted-foreground">"Loading..."</p> }.into_any(),
                    }}
                </Suspense>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn IdentitySummary(
    name: String,
    gender: String,
    age: String,
) -> impl IntoView {
    view! {
        <div class="flex w-full max-w-[550px] flex-col items-center gap-6 lg:gap-8">
            <div class="flex items-center gap-4 lg:gap-6">
                <h1 class="text-4xl font-bold text-custom-primary md:text-5xl">"Hi!"</h1>
                <img src=HELLO_ICON_URL alt="Hello icon" class="h-14 w-14 object-contain md:h-[72px] md:w-[72px]"/>
            </div>

            <div class="w-full max-w-[550px] text-center">
                <div class="space-y-1 md:space-y-2">
                    <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Name"</p>
                    <p class="text-3xl font-light text-custom-primary md:text-4xl">{name}</p>
                </div>

                <div class="mt-6 grid grid-cols-2 gap-6 md:mt-8 md:gap-8">
                    <div class="space-y-1 md:space-y-2">
                        <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Gender"</p>
                        <p class="text-3xl font-light text-custom-primary md:text-4xl">{gender}</p>
                    </div>
                    <div class="space-y-1 md:space-y-2">
                        <p class="text-3xl font-bold text-custom-foreground md:text-4xl">"Age"</p>
                        <p class="text-3xl font-light text-custom-primary md:text-4xl">{age}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmIdentityData {
    pub patient_key: String,
    pub name_snapshot: String,
    pub age_snapshot: i32,
    pub gender_snapshot: String,
}

#[server(GetConfirmIdentityData, "/api")]
pub async fn get_confirm_identity_data(
    patient_key: String,
) -> Result<ConfirmIdentityData, ServerFnError> {
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

        let patient: PatientCase = patient_dsl::patient
            .filter(patient_dsl::patient_key.eq(&patient_key))
            .first(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient query failed: {e}")))?;

        Ok(ConfirmIdentityData {
            patient_key: patient.patient_key,
            name_snapshot: patient.name_snapshot,
            age_snapshot: patient.age_snapshot,
            gender_snapshot: patient.gender_snapshot,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_confirm_identity_data is only available on the server".to_string(),
        ))
    }
}

#[component]
fn ConfirmPanel(
    patient_key: String,
) -> impl IntoView {
    let (need_accessibility, set_need_accessibility) = signal(false);

    view! {
        <div class="flex w-full max-w-[460px] flex-col gap-5 rounded-xl bg-custom-background/25 p-2 md:p-0">
            <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                <input type="checkbox" checked=true class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"/>
                <span class="label-text text-lg font-light text-custom-primary md:text-2xl">
                    "I confirm the personal information is correct"
                </span>
            </label>

            <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"/>
                <span class="label-text text-lg font-light text-custom-primary md:text-2xl">
                    "I agree to the Terms of Service and Privacy Policy."
                </span>
            </label>

            <div class="rounded-[15px] border-2 border-dashed border-info bg-custom-background/40 px-3 py-2 md:px-4 md:py-3">
                <label class="label cursor-pointer justify-start gap-3 md:gap-4">
                    <input
                        type="checkbox"
                        class="checkbox checkbox-primary checkbox-sm md:checkbox-md rounded-[10px]"
                        prop:checked=move || need_accessibility.get()
                        on:change=move |ev| set_need_accessibility.set(event_target_checked(&ev))
                    />
                    <span class="label-text text-lg font-light text-custom-primary md:text-2xl">"I need accessibility layout."</span>
                </label>
            </div>

            <div class="flex justify-center pt-1 md:pt-3">
                <A
                    href=move || {
                        let key = patient_key.clone();
                        let suffix = if key.trim().is_empty() {
                            "".to_string()
                        } else {
                            format!("?patient-id={key}")
                        };
                        if need_accessibility.get() {
                            format!("/app/chat-accessibility{suffix}")
                        } else {
                            format!("/app/chat-default{suffix}")
                        }
                    }
                    attr:class="btn btn-primary h-12 min-h-12 px-8 text-xl font-medium md:h-[87px] md:min-h-[87px] md:px-12 md:text-4xl"
                >
                    "Confirm"
                </A>
            </div>

            <div class="flex justify-center">
                <A href="/app/input-page" attr:class="btn btn-outline">"Back"</A>
            </div>
        </div>
    }
}
