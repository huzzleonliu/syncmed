use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};

const LOGO_GROUP_URL: &str =
    "https://www.figma.com/api/mcp/asset/4ca73d88-9905-4c0a-90d5-59d7045b9d61";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/183ebd1f-0fd1-4713-832f-d3e1f9283e4e";
const SUCCESS_ICON_URL: &str =
    "https://www.figma.com/api/mcp/asset/1163f275-fe7a-4388-ba8e-7286d5b30828";

#[component]
pub fn AppConfirmSuccessPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let duration_text = Resource::new(
        move || patient_key.get(),
        |key| async move {
            let count = get_chat_message_count(key).await?;
            Ok::<_, ServerFnError>(format_duration_from_count(count))
        },
    );

    view! {
        <Title text="App Confirm Success - SyncMed"/>
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

            <section class="mx-auto flex min-h-[calc(100vh-188px)] w-full max-w-[1280px] items-center justify-center px-4 py-8 md:px-6">
                <div class="grid w-full max-w-[1108px] items-center gap-10 lg:grid-cols-2">
                    <div class="flex flex-col items-center gap-8 text-center lg:items-start lg:text-left">
                        <div class="flex flex-col items-center gap-4 lg:flex-row lg:items-center">
                            <img src=SUCCESS_ICON_URL alt="Success icon" class="h-[96px] w-[96px] object-contain"/>
                            <div class="space-y-2">
                                <p class="text-5xl font-bold text-custom-primary">"Thank you"</p>
                                <p class="text-3xl font-semibold text-custom-primary">"The report has been sent to the doctor"</p>
                            </div>
                        </div>

                        <A
                            href=move || {
                                let key = patient_key.get();
                                if key.trim().is_empty() {
                                    "/app/chat-default".to_string()
                                } else {
                                    format!("/app/chat-default?patient-id={key}")
                                }
                            }
                            attr:class="btn btn-neutral border-custom-border bg-custom-background px-8 text-2xl font-light text-custom-foreground"
                        >
                            "↖ Back"
                        </A>
                    </div>

                    <div class="flex flex-col items-center gap-8 text-center">
                        <div class="space-y-3">
                            <p class="text-3xl font-light text-custom-primary">"The report has saved"</p>
                            <p class="text-6xl font-bold text-custom-foreground">
                                <Suspense fallback=move || "1min".to_string()>
                                    {move || match duration_text.get() {
                                        Some(Ok(text)) => text,
                                        Some(Err(_)) => "1min".to_string(),
                                        None => "1min".to_string(),
                                    }}
                                </Suspense>
                            </p>
                            <p class="text-3xl font-light text-custom-primary">"for the diagnose process"</p>
                        </div>

                        <p class="max-w-[520px] text-4xl font-light text-custom-primary">"Please wait patiently for the calling"</p>
                    </div>
                </div>
            </section>

            <footer class="border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

fn format_duration_from_count(chat_count: i64) -> String {
    let total_seconds = chat_count.max(0) * 30;
    let mut total_minutes = (total_seconds + 59) / 60; // round up to minute
    if total_minutes == 0 {
        total_minutes = 1; // keep visible non-zero duration
    }
    if total_minutes < 60 {
        format!("{total_minutes}min")
    } else {
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        format!("{hours}h{minutes}min")
    }
}

#[server(GetChatMessageCount, "/api")]
pub async fn get_chat_message_count(patient_key: String) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{DbPool, schema::{case_chat_messages::dsl as chat_dsl, patient::dsl as patient_dsl}};
        use axum::Extension;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        if patient_key.trim().is_empty() {
            return Ok(0);
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

        let count: i64 = chat_dsl::case_chat_messages
            .filter(chat_dsl::patient_id.eq(patient_id))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("chat count query failed: {e}")))?;

        Ok(count)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = patient_key;
        Err(ServerFnError::new(
            "get_chat_message_count is only available on the server".to_string(),
        ))
    }
}
