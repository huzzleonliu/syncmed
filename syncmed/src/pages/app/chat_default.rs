use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use serde::{Deserialize, Serialize};
use crate::structure::chat::{CaseChatMessageDraft, CaseMedicationPayload};
#[cfg(target_arch = "wasm32")]
use crate::structure::chat::{ChatbotRequest, ChatbotResponse};
#[cfg(target_arch = "wasm32")]
use crate::services::chat::{bootstrap_chat_state, merge_case_medications};
#[cfg(target_arch = "wasm32")]
use crate::services::chat::{save_chat_history, save_local_medications};
#[cfg(target_arch = "wasm32")]
use crate::services::chat::{clear_local_chat_bundle, upload_local_chat_and_medications};

const LOGO_GROUP_URL: &str = "/logo.svg";
const AVATAR_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/ab19cb29-0b34-41d7-94bc-003ced3c67f8";
const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/9a57ef9e-5a9e-45f5-878d-3cd0195c7a06";

#[component]
pub fn AppChatDefaultPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let medications_store = use_context::<RwSignal<Vec<CaseMedicationPayload>>>()
        .unwrap_or_else(|| RwSignal::new(Vec::<CaseMedicationPayload>::new()));
    view! {
        <Title text="App Chat Default - SyncMed"/>
        <main class="h-dvh bg-custom-subtle-background text-custom-foreground flex flex-col">
            <header class="shrink-0 border-b border-custom-border bg-custom-card">
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

            <section class="mx-auto w-full max-w-[1280px] px-4 py-6 md:px-6 md:py-8 flex-1 min-h-0">
                <div class="grid h-full min-h-0 gap-4 lg:grid-cols-[2fr_1fr]">
                    <div class="min-h-0 overflow-auto">
                        <ChatPanel patient_key=patient_key.get_untracked() medications_store=medications_store/>
                    </div>
                    <div class="min-h-0 overflow-auto">
                        <MedicationPanel patient_key=patient_key.get_untracked() medications_store=medications_store/>
                    </div>
                </div>
            </section>

            <footer class="shrink-0 border-t border-custom-border bg-custom-card px-6 py-8 text-center text-sm text-custom-muted-foreground">
                "© 2026 SyncMed. All rights reserved."
            </footer>
        </main>
    }
}

#[component]
fn ChatPanel(
    patient_key: String,
    medications_store: RwSignal<Vec<CaseMedicationPayload>>,
) -> impl IntoView {
    let (draft, set_draft) = signal(String::new());
    let messages_store = use_context::<RwSignal<Vec<CaseChatMessageDraft>>>()
        .unwrap_or_else(|| RwSignal::new(Vec::<CaseChatMessageDraft>::new()));
    let patient_key_for_links = patient_key.clone();
    let patient_key_for_send = patient_key.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (&patient_key_for_send, &medications_store, &messages_store);
    #[cfg(target_arch = "wasm32")]
    {
        bootstrap_chat_state(
            patient_key_for_send.clone(),
            messages_store,
            medications_store,
        );
    }

    view! {
        <div class="card flex h-full min-h-0 flex-col border border-custom-border bg-custom-background shadow-sm">
            <div class="flex items-center justify-between border-b border-custom-border p-4">
                <h2 class="text-xl font-bold text-custom-foreground">"Medication Reconciliation"</h2>
                <div class="flex items-center gap-2">
                    <A
                        href=if patient_key_for_links.trim().is_empty() {
                            "/app/chat-accessibility".to_string()
                        } else {
                            format!("/app/chat-accessibility?patient-id={patient_key_for_links}")
                        }
                        attr:class="btn btn-primary btn-sm"
                    >
                        "Switch to accessibility page"
                    </A>
                    <button type="button" class="btn btn-primary btn-square btn-sm">"↗"</button>
                </div>
            </div>

            <div class="flex-1 min-h-0 space-y-4 overflow-y-auto p-4">
                {move || {
                    let items = messages_store.get();
                    if items.is_empty() {
                        view! { <p class="text-sm text-custom-muted-foreground">"No chat messages yet."</p> }.into_any()
                    } else {
                        items
                            .into_iter()
                            .map(|item| {
                                view! {
                                    <ChatBubble
                                        text=item.content_text
                                        time=item.created_at
                                        from_user=item.sender_type == "patient"
                                    />
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                }}
            </div>

            <div class="mt-auto flex items-center gap-3 border-t border-custom-border p-4">
                <textarea
                    class="textarea textarea-bordered h-20 flex-1 resize-none border-custom-input bg-custom-background"
                    placeholder="Type your message here..."
                    prop:value=move || draft.get()
                    on:input=move |ev| set_draft.set(event_target_value(&ev))
                ></textarea>
                <button
                    type="button"
                    class="btn btn-primary btn-circle"
                    on:click=move |_| {
                        let text = draft.get_untracked().trim().to_string();
                        if text.is_empty() {
                            return;
                        }
                        set_draft.set(String::new());
                        messages_store.update(|items| {
                            items.push(CaseChatMessageDraft {
                                id: None,
                                patient_id: None,
                                sender_type: "patient".to_string(),
                                content_text: text.clone(),
                                created_at: "Now".to_string(),
                            });
                        });
                        #[cfg(target_arch = "wasm32")]
                        save_chat_history(&patient_key_for_send, &messages_store.get_untracked());

                        #[cfg(target_arch = "wasm32")]
                        {
                            let messages_store = messages_store;
                            let key = patient_key_for_send.clone();
                            leptos::task::spawn_local(async move {
                                let payload = ChatbotRequest {
                                    message: text,
                                    patient_id: if key.trim().is_empty() { None } else { Some(key.clone()) },
                                };
                                let request = gloo_net::http::Request::post("http://localhost:3003/api/chat")
                                    .header("content-type", "application/json")
                                    .json(&payload);

                                match request {
                                    Ok(req) => match req.send().await {
                                        Ok(resp) => match resp.json::<ChatbotResponse>().await {
                                            Ok(data) => {
                                                messages_store.update(|items| {
                                                    items.push(CaseChatMessageDraft {
                                                        id: None,
                                                        patient_id: None,
                                                        sender_type: "bot".to_string(),
                                                        content_text: data.reply.clone(),
                                                        created_at: data.timestamp.clone(),
                                                    });
                                                });
                                                save_chat_history(&key, &messages_store.get_untracked());
                                                if let Some(incoming) = data.medications {
                                                    medications_store.update(|local| {
                                                        merge_case_medications(local, incoming)
                                                    });
                                                    save_local_medications(&key, &medications_store.get_untracked());
                                                }
                                            }
                                            Err(err) => {
                                                messages_store.update(|items| {
                                                    items.push(CaseChatMessageDraft {
                                                        id: None,
                                                        patient_id: None,
                                                        sender_type: "bot".to_string(),
                                                        content_text: format!("机器人响应解析失败: {err}"),
                                                        created_at: "Now".to_string(),
                                                    });
                                                });
                                                save_chat_history(&key, &messages_store.get_untracked());
                                            }
                                        },
                                        Err(err) => {
                                            messages_store.update(|items| {
                                                items.push(CaseChatMessageDraft {
                                                    id: None,
                                                    patient_id: None,
                                                    sender_type: "bot".to_string(),
                                                    content_text: format!("请求机器人失败: {err}"),
                                                    created_at: "Now".to_string(),
                                                });
                                            });
                                            save_chat_history(&key, &messages_store.get_untracked());
                                        }
                                    },
                                    Err(err) => {
                                        messages_store.update(|items| {
                                            items.push(CaseChatMessageDraft {
                                                id: None,
                                                patient_id: None,
                                                sender_type: "bot".to_string(),
                                                content_text: format!("构建请求失败: {err}"),
                                                created_at: "Now".to_string(),
                                            });
                                        });
                                        save_chat_history(&key, &messages_store.get_untracked());
                                    }
                                }
                            });
                        }
                    }
                >
                    "➤"
                </button>
                <button type="button" class="btn btn-outline btn-circle border-custom-input">"◍"</button>
            </div>
        </div>
    }
}

#[component]
fn ChatBubble(text: String, time: String, from_user: bool) -> impl IntoView {
    let wrap_class = if from_user {
        "justify-end"
    } else {
        "justify-start"
    };
    let bubble_class = if from_user {
        "bg-primary text-primary-content"
    } else {
        "bg-base-200 text-custom-foreground"
    };

    view! {
        <div class=move || format!("flex {}", wrap_class)>
            <div class=move || format!("max-w-[80%] rounded-lg p-3 {}", bubble_class)>
                <p class="text-sm leading-6 whitespace-pre-wrap">{text}</p>
                <p class="mt-2 text-right text-xs opacity-70">{time}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppChatMessageRow {
    pub sender_type: String,
    pub content_text: String,
    pub created_at_text: String,
}

#[server(GetPatientChatMessages, "/api")]
pub async fn get_patient_chat_messages(
    patient_key: String,
) -> Result<Vec<AppChatMessageRow>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool,
            models::CaseChatMessage,
            schema::{case_chat_messages::dsl as chat_dsl, patient::dsl as patient_dsl},
        };
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

        let patient_id: i32 = patient_dsl::patient
            .filter(patient_dsl::patient_key.eq(&patient_key))
            .select(patient_dsl::id)
            .first(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("patient query failed: {e}")))?;

        let messages: Vec<CaseChatMessage> = chat_dsl::case_chat_messages
            .filter(chat_dsl::patient_id.eq(patient_id))
            .order(chat_dsl::created_at.asc())
            .load(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("chat query failed: {e}")))?;

        Ok(messages
            .into_iter()
            .map(|m| AppChatMessageRow {
                sender_type: m.sender_type,
                content_text: m.content_text,
                created_at_text: m.created_at.format("%H:%M").to_string(),
            })
            .collect())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "get_patient_chat_messages is only available on the server".to_string(),
        ))
    }
}

#[server(AddPatientChatMessage, "/api")]
pub async fn add_patient_chat_message(
    patient_key: String,
    content_text: String,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::db::{
            DbPool,
            models::NewCaseChatMessage,
            schema::{case_chat_messages::dsl as chat_dsl, patient::dsl as patient_dsl},
        };
        use axum::Extension;
        use chrono::Utc;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use leptos_axum::extract;

        let content = content_text.trim().to_string();
        if content.is_empty() {
            return Ok(());
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

        let new_message = NewCaseChatMessage {
            patient_id,
            sender_type: "patient".to_string(),
            content_text: content,
        };

        diesel::insert_into(chat_dsl::case_chat_messages)
            .values(&new_message)
            .execute(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("insert chat failed: {e}")))?;

        diesel::update(patient_dsl::patient.filter(patient_dsl::id.eq(patient_id)))
            .set(patient_dsl::modified_at.eq(Some(Utc::now().naive_utc())))
            .execute(&mut conn)
            .await
            .map_err(|e| ServerFnError::new(format!("update patient modified_at failed: {e}")))?;

        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "add_patient_chat_message is only available on the server".to_string(),
        ))
    }
}

#[component]
fn MedicationPanel(
    patient_key: String,
    medications_store: RwSignal<Vec<CaseMedicationPayload>>,
) -> impl IntoView {
    let messages_store = use_context::<RwSignal<Vec<CaseChatMessageDraft>>>()
        .unwrap_or_else(|| RwSignal::new(Vec::<CaseChatMessageDraft>::new()));
    let (upload_error, set_upload_error) = signal(String::new());
    let (uploading, set_uploading) = signal(false);

    view! {
        <div class="card border border-custom-border bg-custom-background shadow-sm">
            <div class="border-b border-custom-border p-4">
                <h3 class="text-xl font-semibold text-custom-foreground">"Medication List"</h3>
            </div>

            <div class="p-4">
                <div class="rounded-lg border-2 border-dashed border-custom-input bg-custom-card p-2">
                    {move || {
                        let items = medications_store.get();
                        if items.is_empty() {
                            view! { <p class="p-3 text-sm text-custom-muted-foreground">"No matched medications yet."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-3">
                                    {items
                                        .into_iter()
                                        .map(|med| view! { <MedicineCard medication=med/> })
                                        .collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>

            <div class="p-4 pt-0">
                <A
                    href="#"
                    attr:class=move || {
                        if uploading.get() {
                            "btn btn-disabled w-full".to_string()
                        } else {
                            "btn btn-primary w-full".to_string()
                        }
                    }
                    on:click=move |ev| {
                        ev.prevent_default();
                        if uploading.get_untracked() {
                            return;
                        }
                        set_upload_error.set(String::new());
                        set_uploading.set(true);
                        let key = patient_key.clone();
                        let messages = messages_store.get_untracked();
                        let medications = medications_store.get_untracked();
                        #[cfg(not(target_arch = "wasm32"))]
                        let _ = (
                            key,
                            messages,
                            medications,
                            &set_upload_error,
                            &set_uploading,
                        );

                        #[cfg(target_arch = "wasm32")]
                        {
                            let messages_store = messages_store;
                            let medications_store = medications_store;
                            let set_upload_error = set_upload_error;
                            let set_uploading = set_uploading;
                            leptos::task::spawn_local(async move {
                                let result = upload_local_chat_and_medications(
                                    key.clone(),
                                    messages,
                                    medications,
                                )
                                .await;
                                match result {
                                    Ok(_) => {
                                        clear_local_chat_bundle(&key);
                                        messages_store.set(Vec::new());
                                        medications_store.set(Vec::new());
                                        if let Some(window) = web_sys::window() {
                                            let target = if key.trim().is_empty() {
                                                "/app/confirm-success-page".to_string()
                                            } else {
                                                format!("/app/confirm-success-page?patient-id={key}")
                                            };
                                            let _ = window.location().set_href(&target);
                                        }
                                    }
                                    Err(err) => {
                                        set_upload_error.set(format!("Upload failed: {err}"));
                                    }
                                }
                                set_uploading.set(false);
                            });
                        }
                    }
                >
                    "Confirm and Upload"
                </A>
                <Show when=move || !upload_error.get().is_empty()>
                    <p class="mt-2 text-center text-sm text-red-600">{move || upload_error.get()}</p>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn MedicineCard(medication: CaseMedicationPayload) -> impl IntoView {
    let taking_period = format!(
        "{} -> {}",
        medication.start_date.as_deref().unwrap_or("-"),
        medication.end_date.as_deref().unwrap_or("ongoing"),
    );
    let comments = medication
        .notes
        .clone()
        .unwrap_or_else(|| "No comments".to_string());

    view! {
        <div class="card border border-custom-border bg-custom-card">
            <div class="card-body gap-3 p-4">
                <div class="flex items-start gap-3">
                    <img src=MEDICINE_IMAGE_URL alt="Medicine" class="h-[70px] w-[70px] rounded bg-custom-background object-cover p-2"/>
                    <div class="flex-1">
                        <h4 class="text-lg font-bold text-custom-foreground">{medication.med_name.clone()}</h4>
                        <p class="text-base text-custom-primary">"Dose: "{medication.dose.clone()}</p>
                        <p class="text-sm text-custom-primary">"Frequency: "{medication.frequency.clone()}</p>
                    </div>
                    <span class="badge badge-primary">"Generated"</span>
                </div>

                <div class="grid grid-cols-2 gap-3 text-sm">
                    <div>
                        <p class="font-semibold text-custom-foreground">"Taking period"</p>
                        <p class="text-custom-foreground/80">{taking_period}</p>
                    </div>
                    <div>
                        <p class="font-semibold text-custom-foreground">"patient_id"</p>
                        <p class="text-custom-foreground/80">{medication.patient_id}</p>
                    </div>
                </div>

                <div>
                    <p class="mb-1 text-sm font-semibold text-custom-foreground">"Comments"</p>
                    <div class="rounded border-2 border-custom-ring bg-custom-background p-2 text-sm text-custom-foreground">
                        {comments}
                    </div>
                </div>
            </div>
        </div>
    }
}
