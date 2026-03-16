use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use crate::structure::chat::{CaseChatMessageDraft, CaseMedicationPayload};
#[cfg(target_arch = "wasm32")]
use crate::services::chat::{clear_local_chat_bundle, upload_local_chat_and_medications};

const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/805648dc-bced-4bde-bf77-9325d2c0ce72";

#[component]
pub fn AppChatConfirmPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let messages_store = use_context::<RwSignal<Vec<CaseChatMessageDraft>>>()
        .unwrap_or_else(|| RwSignal::new(Vec::<CaseChatMessageDraft>::new()));
    let medications_store = use_context::<RwSignal<Vec<CaseMedicationPayload>>>()
        .unwrap_or_else(|| RwSignal::new(Vec::<CaseMedicationPayload>::new()));
    let (upload_error, set_upload_error) = signal(String::new());
    let (uploading, set_uploading) = signal(false);

    view! {
        <Title text="Chat Confirm - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <section class="mx-auto w-full max-w-[1024px] px-2 py-2 md:px-4 md:py-4">
                <div class="card border border-custom-ring bg-custom-background shadow-md">
                    <div class="border-b border-custom-border p-4 text-center">
                        <h2 class="text-5xl font-semibold">"Medication List"</h2>
                    </div>

                    <div class="p-6">
                        <div class="rounded-lg border-2 border-dashed border-custom-border bg-custom-card p-4">
                            {move || {
                                let items = medications_store.get();
                                if items.is_empty() {
                                    view! {
                                        <p class="p-3 text-center text-xl text-custom-muted-foreground">
                                            "No matched medications yet."
                                        </p>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 lg:gap-6">
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

                    <div class="pb-6 text-center">
                        <div class="flex items-center justify-center gap-4">
                            <A
                                href=move || {
                                    let key = patient_key.get();
                                    if key.trim().is_empty() {
                                        "/app/chat-default".to_string()
                                    } else {
                                        format!("/app/chat-default?patient-id={key}")
                                    }
                                }
                                attr:class="btn btn-outline px-8 text-2xl font-bold md:text-3xl"
                            >
                                "Back"
                            </A>
                            <A
                                href="#"
                                attr:class=move || {
                                    if uploading.get() {
                                        "btn btn-disabled px-8 text-2xl font-bold md:text-3xl".to_string()
                                    } else {
                                        "btn btn-primary px-8 text-2xl font-bold md:text-3xl".to_string()
                                    }
                                }
                                on:click=move |ev| {
                                    ev.prevent_default();
                                    if uploading.get_untracked() {
                                        return;
                                    }
                                    set_upload_error.set(String::new());
                                    set_uploading.set(true);
                                    let key = patient_key.get_untracked();
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
                        </div>
                        <Show when=move || !upload_error.get().is_empty()>
                            <p class="mt-4 text-center text-sm text-red-600">{move || upload_error.get()}</p>
                        </Show>
                    </div>
                </div>
            </section>
        </main>
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
            <div class="card-body gap-4 p-4 lg:gap-3 lg:p-5">
                <div class="flex items-start gap-3">
                    <img src=MEDICINE_IMAGE_URL alt="Medicine" class="h-[70px] w-[80px] object-cover lg:h-[55px] lg:w-[70px]"/>
                    <div class="flex-1">
                        <p class="text-6xl font-bold text-custom-foreground lg:text-5xl">{medication.med_name}</p>
                        <p class="text-4xl text-custom-primary lg:text-3xl">"Dose: "{medication.dose}</p>
                        <p class="text-5xl font-light text-custom-primary lg:text-4xl">"Frequency: "{medication.frequency}</p>
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-6 text-4xl lg:gap-3 lg:text-2xl">
                    <div>
                        <p class="font-semibold">"Taking period"</p>
                        <p class="font-normal">{taking_period}</p>
                    </div>
                    <div>
                        <p class="font-semibold">"patient_id"</p>
                        <p class="font-normal">{medication.patient_id}</p>
                    </div>
                </div>

                <div>
                    <p class="mb-2 text-4xl font-semibold lg:text-2xl">"Comments"</p>
                    <div class="rounded border-2 border-custom-ring bg-custom-background p-2 text-4xl lg:min-h-[56px] lg:text-2xl">
                        {comments}
                    </div>
                </div>
            </div>
        </div>
    }
}
