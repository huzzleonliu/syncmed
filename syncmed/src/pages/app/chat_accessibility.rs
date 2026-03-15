use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_query_map};
use crate::pages::app::chat_default::{
    add_patient_chat_message, get_patient_chat_messages,
};

const MEDICINE_IMAGE_URL: &str =
    "https://www.figma.com/api/mcp/asset/592defbd-fa4b-49d4-b550-c6a4ee112841";

#[component]
pub fn AppChatAccessibilityPage() -> impl IntoView {
    let query = use_query_map();
    let patient_key = Memo::new(move |_| {
        query
            .get()
            .get("patient-id")
            .unwrap_or_else(String::new)
    });
    let (draft, set_draft) = signal(String::new());
    let (reload_key, set_reload_key) = signal(0_u64);
    let patient_key_for_resource = patient_key.clone();
    let patient_key_for_send = patient_key.clone();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &add_patient_chat_message;
    #[cfg(not(target_arch = "wasm32"))]
    let _ = &set_reload_key;
    let chat_history = Resource::new(
        move || (patient_key_for_resource.get(), reload_key.get()),
        |(key, _)| async move {
            if key.trim().is_empty() {
                return Err(ServerFnError::new("Missing patient-id in URL"));
            }
            get_patient_chat_messages(key).await
        },
    );

    view! {
        <Title text="App Chat Accessibility - SyncMed"/>
        <main class="min-h-screen bg-custom-subtle-background text-custom-foreground">
            <section class="mx-auto w-full max-w-[1326px] px-2 py-2 md:px-4 md:py-4">
                <div class="card overflow-hidden border border-custom-ring bg-custom-background shadow-md">
                    <div class="flex flex-col gap-3 border-b border-custom-border p-3 md:flex-row md:items-center md:justify-between md:p-4">
                        <h2 class="text-xl font-bold md:text-2xl">"Medication Reconciliation"</h2>
                        <div class="flex items-center gap-2 self-end md:self-auto">
                            <A
                                href=move || {
                                    let key = patient_key.get();
                                    if key.trim().is_empty() {
                                        "/app/chat-default".to_string()
                                    } else {
                                        format!("/app/chat-default?patient-id={key}")
                                    }
                                }
                                attr:class="btn btn-primary btn-sm"
                            >
                                "Switch to default page"
                            </A>
                            <A
                                href=move || {
                                    let key = patient_key.get();
                                    if key.trim().is_empty() {
                                        "/app/chat-default".to_string()
                                    } else {
                                        format!("/app/chat-default?patient-id={key}")
                                    }
                                }
                                attr:class="btn btn-primary btn-square btn-sm"
                            >
                                "↗"
                            </A>
                        </div>
                    </div>

                    <div class="bg-custom-subtle-background p-2 md:p-3">
                        <div class="rounded-lg bg-custom-background lg:grid lg:min-h-[620px] lg:grid-cols-2 lg:gap-6">
                            <div class="space-y-1 p-1 md:space-y-2 md:p-2 lg:space-y-2 lg:p-3">
                                <Suspense fallback=move || view! { <p class="text-sm text-custom-muted-foreground">"Loading chat..."</p> }>
                                    {move || match chat_history.get() {
                                        Some(Ok(items)) => {
                                            if items.is_empty() {
                                                view! { <p class="text-sm text-custom-muted-foreground">"No chat messages yet."</p> }.into_any()
                                            } else {
                                                items
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(idx, item)| {
                                                        view! {
                                                            <AccBubble
                                                                text=item.content_text
                                                                time=item.created_at_text
                                                                user=item.sender_type == "patient"
                                                                with_medicine=idx == 0 && item.sender_type != "patient"
                                                            />
                                                        }
                                                    })
                                                    .collect_view()
                                                    .into_any()
                                            }
                                        }
                                        Some(Err(err)) => view! {
                                            <p class="text-sm text-red-600">{format!("Failed to load chat: {err}")}</p>
                                        }.into_any(),
                                        None => view! { <p class="text-sm text-custom-muted-foreground">"Loading chat..."</p> }.into_any(),
                                    }}
                                </Suspense>
                            </div>

                            <div class="mt-2 border-t border-custom-border p-3 md:p-4 lg:mt-0 lg:border-l lg:border-t-0 lg:p-6">
                                <div class="mx-auto grid w-fit grid-cols-3 gap-x-3 gap-y-2 md:gap-x-8">
                                    <KeyPadButton main="1" sub=""/>
                                    <KeyPadButton main="2" sub="ABC"/>
                                    <KeyPadButton main="3" sub="DEF"/>
                                    <KeyPadButton main="4" sub="GHI"/>
                                    <KeyPadButton main="5" sub="JKL"/>
                                    <KeyPadButton main="6" sub="MNO"/>
                                    <KeyPadButton main="7" sub="PQRS"/>
                                    <KeyPadButton main="8" sub="TUV"/>
                                    <KeyPadButton main="9" sub="WXYZ"/>
                                    <KeyPadButton main="*" sub=""/>
                                    <KeyPadButton main="0" sub="+"/>
                                    <KeyPadButton main="#" sub=""/>
                                </div>

                                <div class="mt-3 flex justify-center lg:mt-10">
                                    <div class="mr-2 flex-1">
                                        <input
                                            type="text"
                                            class="input input-bordered w-full border-custom-input bg-custom-background text-custom-foreground"
                                            placeholder="Type message..."
                                            prop:value=move || draft.get()
                                            on:input=move |ev| set_draft.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        on:click=move |_| {
                                            let key = patient_key_for_send.get_untracked();
                                            let text = draft.get_untracked().trim().to_string();
                                            if key.trim().is_empty() || text.is_empty() {
                                                return;
                                            }
                                            set_draft.set(String::new());
                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                leptos::task::spawn_local(async move {
                                                    let _ = add_patient_chat_message(key, text).await;
                                                    set_reload_key.update(|v| *v += 1);
                                                });
                                            }
                                        }
                                    >
                                        "Send"
                                    </button>
                                    <A
                                        href=move || {
                                            let key = patient_key.get();
                                            if key.trim().is_empty() {
                                                "/app/chat-confirm".to_string()
                                            } else {
                                                format!("/app/chat-confirm?patient-id={key}")
                                            }
                                        }
                                        attr:class="btn h-14 min-h-14 rounded-full border-none bg-custom-accent px-8 text-2xl font-medium text-custom-foreground hover:bg-custom-accent md:h-16 md:min-h-16 md:px-10 md:text-4xl"
                                    >
                                        "End Chat"
                                    </A>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
fn AccBubble(
    text: String,
    time: String,
    user: bool,
    with_medicine: bool,
) -> impl IntoView {
    let bubble_class = if user {
        "bg-custom-input"
    } else {
        "bg-base-300"
    };
    view! {
        <div class=move || format!("rounded-lg border border-custom-border px-4 py-3 md:px-7 md:py-4 lg:max-w-[560px] lg:px-10 {}", bubble_class)>
            <p class="text-lg leading-[1.35] md:text-2xl">{text}</p>
            {move || if with_medicine {
                view! {
                    <div class="mt-3 flex items-start gap-4">
                        <img src=MEDICINE_IMAGE_URL alt="Medicine preview" class="h-[64px] w-[58px] object-cover md:h-[78px] md:w-[70px]"/>
                        <div>
                            <p class="text-2xl font-bold text-custom-foreground md:text-4xl lg:text-5xl">"medicine name"</p>
                            <p class="text-xl font-light text-custom-primary md:text-3xl lg:text-4xl">"company name"</p>
                            <p class="text-lg text-custom-primary md:text-2xl lg:text-4xl">"Function : Fever reducer, pain reliever"</p>
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            <p class="mt-2 text-right text-sm text-custom-muted-foreground">{time}</p>
        </div>
    }
}

#[component]
fn KeyPadButton(main: &'static str, sub: &'static str) -> impl IntoView {
    view! {
        <button type="button" class="btn h-14 min-h-14 w-14 rounded-full border-none bg-custom-accent p-0 text-custom-foreground hover:bg-custom-accent md:h-20 md:min-h-20 md:w-20 lg:h-[90px] lg:min-h-[90px] lg:w-[90px]">
            <span class="flex flex-col items-center leading-none">
                <span class="text-xl md:text-3xl lg:text-4xl">{main}</span>
                <span class="text-[10px] md:text-xs">{sub}</span>
            </span>
        </button>
    }
}
